use implicit_clone::sync::IArray;
use implicit_clone::sync::IMap;
use implicit_clone::ImplicitClone;
use itertools::Itertools;

use crate::language::Language;
use crate::language::LanguageStore;
use crate::meta::MetaStore;
use crate::utils::all_path_ids;
use crate::utils::all_possible_indices;
use crate::utils::sync::path::IPath;

#[derive(Debug, Clone)]
pub struct PageStore {
    pub pages: IMap<IPath, IMap<Language, IPath>>,
}

impl PageStore {
    pub fn iter(&self) -> impl Iterator<Item = (&IPath, &IMap<Language, IPath>)> + '_ {
        self.pages.iter()
    }

    pub fn get(&self, path: IPath) -> Option<IMap<Language, IPath>> {
        self.pages.get(&path)
    }
}

impl ImplicitClone for PageStore {}

pub fn process_pages(
    src_dir_path: IPath,
    languages: LanguageStore,
    metas: MetaStore, // FIXME check if page is valid
) -> PageStore {
    PageStore {
        pages: languages
            .into_iter()
            .flat_map(|lang| {
                all_path_ids(src_dir_path.clone()).map(move |path| (lang.clone(), path))
            })
            .flat_map(|(lang, path_id)| {
                assert!(
                    metas.get(path_id.clone()).is_some(),
                    "missing meta for page `{}`",
                    path_id.display()
                );

                let indices =
                    all_possible_indices(src_dir_path.clone(), path_id.clone(), lang.id.clone())
                        .collect::<IArray<_>>();
                assert!(
                    indices.len() <= 1,
                    "there should not be multiple `{}` page files for the path `{}`",
                    lang.id,
                    path_id.display()
                );
                indices
                    .into_iter()
                    .map(move |index| (lang.clone(), path_id.clone(), index))
            })
            .into_group_map_by(|(_, path_id, _)| path_id.clone())
            .into_iter()
            .map(|(path_id, indices)| {
                (
                    path_id,
                    indices
                        .into_iter()
                        .map(|(lang, _, index)| (lang, index))
                        .collect(),
                )
            })
            .collect(),
    }
}
