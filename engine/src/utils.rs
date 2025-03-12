use std::fmt::Debug;
use std::hash::Hash;
use std::iter;
use std::rc::Rc;
use std::sync::Arc;

use implicit_clone::sync::IArray;
use implicit_clone::sync::IMap;
use implicit_clone::sync::IString;
use implicit_clone::ImplicitClone;
use itertools::Itertools;
use serde::Deserialize;
use serde::Serialize;

use crate::sync::path::CollectIPath;
use crate::sync::path::IPath;

//
// fs utils
//

/// Iterate over all directory files recursively (the directories themselves are not listed).
pub fn iter_deep(dir: IPath) -> impl Iterator<Item = IPath> {
    dir.read_dir()
        .ok()
        .unwrap_or_else(|| panic!("failed to iter_deep dir `{}`", dir.display()))
        .flat_map::<Box<dyn Iterator<Item = IPath>>, _>(move |path| {
            let path = path.unwrap();
            match path.is_dir() {
                true => Box::new(iter_deep(path)),
                false => {
                    debug_assert_ne!(path, IPath::default());
                    debug_assert_ne!(path, dir);
                    Box::new(iter::once(path))
                }
            }
        })
}

/// Split a dotted filename like `a.b.c.d` into `a.b`, `c` and `d`, where `a.b` is the name, `c` is
/// the pre-extension part and `d` is the extension. The returned `c` and `d` can be empty.
pub fn split_into_name_pre_ext(filename: IString) -> (IString, IString, IString) {
    let filename = filename.as_str();
    // TODO do the splitting by known extensions first (e.g. `tar.gz`, etc), fallback to rsplit_once
    let (stem, ext) = filename.rsplit_once('.').unwrap_or((filename, ""));
    let (name, pre) = stem.rsplit_once('.').unwrap_or((stem, ""));
    (name.into(), pre.into(), ext.into())
}

/// Convert a path like `a/b/c/file.pre.html` to pair `([a, b, c], file)`. Empty path is converted
/// to a pair `([], "")`.
pub fn path_to_parts_and_first(path: IPath) -> (IArray<IString>, IString) {
    // a/b/c/file.pre.html -> [a, b, c, file.pre.html]
    let mut path_parts = path.into_iter_lossy().collect_vec();
    // [a, b, c, file.pre.html] -> ([a, b, c], file.pre.html)
    let last = path_parts.pop().unwrap_or_default();
    // file.pre.html -> [file, pre, html]
    let (first_part, ..) = split_into_name_pre_ext(last);
    (path_parts.into(), first_part)
}

/// Gets all unique paths like `a/b/c` from paths `a/b/c.d.*` and `a/b/c/index.d.*` for all files in
/// a directory, recursively.
///
/// The resulting paths are in relativity to `dir`. `index.d.*` maps to an empty path.
pub fn all_path_ids(dir: IPath) -> impl Iterator<Item = IPath> {
    iter_deep(dir.clone())
        .map(move |path| {
            debug_assert_ne!(path, dir);
            debug_assert_ne!(path, IPath::default());

            let path = path.strip_prefix(dir.clone()).unwrap();
            let (parts, first) = path_to_parts_and_first(path);
            let mut parts = parts.to_vec();
            if first != "index" {
                parts.push(first.clone());
            }
            parts.iter().collect_ipath()
        })
        .unique()
}

/// Get all filepaths representing `root/a/b/c.d.*`, where:
///
/// - `a/b/c` is the `path` - both all `root/a/b/c.d.*` and all `root/a/b/c/index.d.*` are listed
/// - `d` is the `pre_extension_part`
/// - `*` is an extension - all extensions are listed in the returned iterator
///
/// There can be multiple files differing only by extension part, as well as by `/index` addition.
/// Returned iterator lists all these, as long as they actually exist in the filesystem. Mind that
/// if this iterator does actually list multiple files, they might be interpreted as *clashing* with
/// each other (e.g. `root/a/b/c/file.test.html` and `root/a/b/c/file.test.md` and
/// `root/a/b/c/file/index.test.html` and `root/a/b/c/file/index.test.md` might all be considered
/// clashing, as they all might be defining same thing).
pub fn all_possible_indices(
    root: IPath,
    path_id: IPath,
    pre_extension_part: IString,
) -> impl Iterator<Item = IPath> {
    // a/b/c/file.pre.html -> ([a, b, c], file)
    let (path_parts, first_part) = path_to_parts_and_first(path_id.clone());
    // [a, b, c] + [file] -> a/b/c/file
    let reassembled_path = path_parts
        .iter()
        .cloned()
        .chain(iter::once(first_part.clone()))
        .collect::<IPath>();

    // try to find root/a/b/c/file/index.pre.*
    let reassembled_path_empty = reassembled_path == IPath::default();
    let reassembled_path = root.join(reassembled_path);
    (reassembled_path.is_dir() || reassembled_path_empty)
        .then(|| {
            reassembled_path.read_dir().unwrap().unwrapping().filter({
                let pre_extension_part = pre_extension_part.clone();
                move |f| {
                    let (name, pre, _) = split_into_name_pre_ext(f.file_name_lossy().unwrap());
                    name == "index" && pre == pre_extension_part
                }
            })
        })
        .into_iter()
        .flatten()
        // try to find root/a/b/c/file.pre.*
        .chain({
            // look into root/a/b/c
            let path = path_parts.into_iter().collect::<IPath>();
            root.join(&path)
                .read_dir()
                .ok()
                .unwrap_or_else(|| {
                    panic!(
                        "failed to read dir `{}` via all_possible_indices",
                        path.display()
                    )
                })
                .unwrapping()
                .filter(move |f| {
                    let (name, pre, _) = split_into_name_pre_ext(f.file_name_lossy().unwrap());
                    name == first_part && pre == pre_extension_part
                })
        })
}

//
// info
//

/// `title` and `description` of any part that may contain them (tag, page).
///
/// The `T` param is usually `IString` (for required defaults) or `Option<IString>` (for optional
/// translations).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Info<T> {
    pub title: T,
    pub description: T,
}

impl<T: ImplicitClone> ImplicitClone for Info<T> {}

impl<T: Serialize> From<Info<T>> for upon::Value {
    fn from(value: Info<T>) -> Self {
        let Info { title, description } = value;
        upon::value! {
            title: title,
            description: description
        }
    }
}

//
// traits
//

pub trait Unwrap {
    type Output;

    fn unwrap(self) -> Self::Output;
}

impl<T> Unwrap for Option<T> {
    type Output = T;

    fn unwrap(self) -> Self::Output {
        self.unwrap()
    }
}

impl<T, E: Debug> Unwrap for Result<T, E> {
    type Output = T;

    fn unwrap(self) -> Self::Output {
        self.unwrap()
    }
}

pub trait Unwrapping {
    type Output;

    fn unwrapping(self) -> Self::Output;
}

impl<T: Unwrap, I: Iterator<Item = T>> Unwrapping for I {
    type Output = iter::Map<I, fn(T) -> T::Output>;

    fn unwrapping(self) -> Self::Output {
        self.map(Unwrap::unwrap)
    }
}

pub trait ToRc {
    fn to_rc(&self) -> Rc<Self>;
}

impl<T: ?Sized> ToRc for T
where
    Rc<T>: for<'a> From<&'a T>,
{
    fn to_rc(&self) -> Rc<Self> {
        self.into()
    }
}

pub trait ToArc {
    fn to_arc(&self) -> Arc<Self>;
}

impl<T: ?Sized> ToArc for T
where
    Arc<T>: for<'a> From<&'a T>,
{
    fn to_arc(&self) -> Arc<Self> {
        self.into()
    }
}

pub trait GetRef<K, V> {
    fn get_ref<'a>(&'a self, key: &K) -> Option<&'a V>;
}

impl<K, V> GetRef<K, V> for IMap<K, V>
where
    K: Eq + Hash + ImplicitClone,
    V: PartialEq + ImplicitClone,
{
    fn get_ref<'a>(&'a self, key: &K) -> Option<&'a V> {
        match self {
            IMap::Static(a) => a.iter().find_map(|(k, v)| (k == key).then_some(v)),
            IMap::Rc(a) => a.get(key),
        }
    }
}
