use std::cmp::Ordering;
use std::ffi::OsStr;
use std::io;
use std::path::Display;
use std::path::Path;
use std::path::PathBuf;
use std::path::StripPrefixError;

use implicit_clone::sync::IString;
use implicit_clone::ImplicitClone;

use crate::utils::ToArc;

use super::Rc;

#[derive(Debug, Clone)]
pub enum IPath {
    Static(&'static Path),
    Rc(Rc<Path>),
}

impl IPath {
    pub fn new(s: &'static (impl AsRef<OsStr> + ?Sized)) -> IPath {
        IPath::Static(Path::new(s))
    }

    pub fn as_path(&self) -> &Path {
        match self {
            IPath::Static(path) => path,
            IPath::Rc(path) => path.as_ref(),
        }
    }

    pub fn display(&self) -> Display<'_> {
        self.as_path().display()
    }

    pub fn file_name_lossy(&self) -> Option<IString> {
        self.as_path()
            .file_name()
            .map(|name| name.to_string_lossy().to_arc().into())
    }

    pub fn to_string_lossy(&self) -> IString {
        self.as_path().to_string_lossy().to_arc().into()
    }

    pub fn canonicalize(&self) -> io::Result<IPath> {
        self.as_path().canonicalize().map(|path| path.to_ipath())
    }

    pub fn read_dir(&self) -> io::Result<impl Iterator<Item = io::Result<IPath>>> {
        self.as_path()
            .read_dir()
            .map(|iter| iter.map(|entry| entry.map(|entry| entry.path().to_ipath())))
    }

    pub fn into_iter_lossy(&self) -> impl Iterator<Item = IString> + '_ {
        self.as_path()
            .iter()
            .map(|component| component.to_string_lossy().to_arc().into())
    }

    pub fn strip_prefix(&self, prefix: IPath) -> Result<IPath, StripPrefixError> {
        self.as_path()
            .strip_prefix(prefix.as_path())
            .map(|path| path.to_ipath())
    }

    pub fn join(&self, path: impl AsRef<Path>) -> IPath {
        self.as_path().join(path.as_ref()).to_ipath()
    }
}

impl ImplicitClone for IPath {}

impl Default for IPath {
    fn default() -> Self {
        IPath::Static(Path::new(""))
    }
}

impl AsRef<Path> for IPath {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

impl std::hash::Hash for IPath {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_path(), state)
    }
}

impl std::borrow::Borrow<Path> for IPath {
    fn borrow(&self) -> &Path {
        self.as_path()
    }
}

impl std::ops::Deref for IPath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.as_path()
    }
}

impl From<&'static Path> for IPath {
    fn from(path: &'static Path) -> Self {
        IPath::Static(path)
    }
}

impl From<PathBuf> for IPath {
    fn from(path: PathBuf) -> Self {
        IPath::Rc(Rc::from(path))
    }
}

impl From<Rc<Path>> for IPath {
    fn from(path: Rc<Path>) -> Self {
        IPath::Rc(path)
    }
}

impl From<&Self> for IPath {
    fn from(p: &Self) -> Self {
        p.clone()
    }
}

macro_rules! impl_cmp_as_str {
    (PartialEq::<$type1:ty, $type2:ty>) => {
        impl_cmp_as_str!(PartialEq::<$type1, $type2>::eq -> bool);
    };
    (PartialOrd::<$type1:ty, $type2:ty>) => {
        impl_cmp_as_str!(PartialOrd::<$type1, $type2>::partial_cmp -> Option<Ordering>);
    };
    ($trait:ident :: <$type1:ty, $type2:ty> :: $fn:ident -> $ret:ty) => {
        impl $trait<$type2> for $type1 {
            fn $fn(&self, other: &$type2) -> $ret {
                $trait::$fn(AsRef::<Path>::as_ref(self), AsRef::<Path>::as_ref(other))
            }
        }
    };
}

impl Eq for IPath {}

impl_cmp_as_str!(PartialEq::<IPath, IPath>);
impl_cmp_as_str!(PartialEq::<IPath, Path>);
impl_cmp_as_str!(PartialEq::<Path, IPath>);
impl_cmp_as_str!(PartialEq::<IPath, &Path>);
impl_cmp_as_str!(PartialEq::<&Path, IPath>);
impl_cmp_as_str!(PartialEq::<IPath, PathBuf>);
impl_cmp_as_str!(PartialEq::<PathBuf, IPath>);
impl_cmp_as_str!(PartialEq::<IPath, &PathBuf>);
impl_cmp_as_str!(PartialEq::<&PathBuf, IPath>);

impl Ord for IPath {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Ord::cmp(AsRef::<Path>::as_ref(self), AsRef::<Path>::as_ref(other))
    }
}

// Manual implementation of PartialOrd that uses Ord to ensure it is consistent, as
// recommended by clippy.
impl PartialOrd for IPath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl_cmp_as_str!(PartialOrd::<IPath, Path>);
impl_cmp_as_str!(PartialOrd::<Path, IPath>);
impl_cmp_as_str!(PartialOrd::<IPath, &Path>);
impl_cmp_as_str!(PartialOrd::<&Path, IPath>);
impl_cmp_as_str!(PartialOrd::<IPath, PathBuf>);
impl_cmp_as_str!(PartialOrd::<PathBuf, IPath>);
impl_cmp_as_str!(PartialOrd::<IPath, &PathBuf>);
impl_cmp_as_str!(PartialOrd::<&PathBuf, IPath>);

impl serde::Serialize for IPath {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        <Path as serde::Serialize>::serialize(self, serializer)
    }
}

impl<'de> serde::Deserialize<'de> for IPath {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <PathBuf as serde::Deserialize>::deserialize(deserializer).map(IPath::from)
    }
}

impl<S: AsRef<str>> FromIterator<S> for IPath {
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        iter.into_iter()
            .map(|s| s.as_ref().to_string())
            .collect::<PathBuf>()
            .to_ipath()
    }
}

pub trait CollectIPath {
    fn collect_ipath(self) -> IPath;
}

impl<S: AsRef<str>, I: Iterator<Item = S>> CollectIPath for I {
    fn collect_ipath(self) -> IPath {
        self.collect()
    }
}

pub trait ToIPath {
    fn to_ipath(&self) -> IPath;
}

impl<T: AsRef<Path>> ToIPath for T {
    fn to_ipath(&self) -> IPath {
        IPath::from(Rc::from(self.as_ref()))
    }
}
