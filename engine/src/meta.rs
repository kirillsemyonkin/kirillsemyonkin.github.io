use std::fs;
use std::ops::Index;

use implicit_clone::sync::IArray;
use implicit_clone::sync::IMap;
use implicit_clone::sync::IString;
use implicit_clone::ImplicitClone;
use itertools::Itertools;

use crate::language::Language;
use crate::language::LanguageStore;
use crate::tag::Tag;
use crate::tag::TagStore;
use crate::utils::all_path_ids;
use crate::utils::all_possible_indices;
use crate::sync::path::IPath;
use crate::utils::GetRef;
use crate::utils::Info;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Meta {
    pub path: IPath,
    pub tags: IArray<Tag>,
    pub default_info: Info<IString>,
    pub infos: IMap<Language, Info<Option<IString>>>,
}

impl Meta {
    pub fn info(&self, lang: Language) -> Info<IString> {
        self.infos
            .get(&lang)
            .map(|info| Info {
                title: info
                    .title
                    .unwrap_or_else(|| self.default_info.title.clone()),
                description: info
                    .description
                    .unwrap_or_else(|| self.default_info.description.clone()),
            })
            .unwrap_or_else(|| self.default_info.clone())
    }

    pub fn title(&self, lang: Language) -> IString {
        self.infos
            .get(&lang)
            .and_then(|info| info.title)
            .unwrap_or_else(|| self.default_info.title.clone())
    }

    pub fn description(&self, lang: Language) -> IString {
        self.infos
            .get(&lang)
            .and_then(|info| info.description)
            .unwrap_or_else(|| self.default_info.description.clone())
    }
}

impl ImplicitClone for Meta {}

#[derive(Debug, Clone)]
pub struct MetaStore {
    pub metas: IMap<IPath, Meta>,
}

impl MetaStore {
    pub fn get(&self, path: IPath) -> Option<Meta> {
        self.metas.get(&path)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Meta> + '_ {
        self.metas.values()
    }

    pub fn iter_ids(&self) -> impl Iterator<Item = &IPath> + '_ {
        self.metas.keys()
    }

    pub fn iter_by_tag(&self, tag: Tag) -> impl Iterator<Item = &Meta> + '_ {
        self.iter().filter(move |meta| meta.tags.contains(&tag))
    }

    pub fn title(&self, path: IPath, lang: Language) -> Option<IString> {
        self.metas.get(&path).map(|meta| meta.title(lang))
    }

    pub fn description(&self, path: IPath, lang: Language) -> Option<IString> {
        self.metas.get(&path).map(|meta| meta.description(lang))
    }
}

impl ImplicitClone for MetaStore {}

impl Index<IPath> for MetaStore {
    type Output = Meta;

    fn index(&self, index: IPath) -> &Self::Output {
        self.metas.get_ref(&index).unwrap()
    }
}

pub fn process_metas(
    source_dir_path: IPath,
    languages: LanguageStore,
    tags: TagStore,
) -> MetaStore {
    MetaStore {
        metas: all_path_ids(source_dir_path.clone())
            .map(|path_id| {
                let indices =
                    all_possible_indices(source_dir_path.clone(), path_id.clone(), "meta".into())
                        .collect_vec();
                assert!(
                    !indices.is_empty(),
                    "missing meta file for the page path `{}`",
                    path_id.display()
                );
                assert!(
                    indices.len() == 1,
                    "there should be exactly 1 page meta file for the page path `{}`, found: {}",
                    path_id.display(),
                    indices
                        .iter()
                        .map(|index| format!("`{}`", index.display()))
                        .join(", ")
                );
                (path_id, indices.into_iter().next().unwrap())
            })
            .map(|(path_id, path)| {
                let mut table =
                    toml::from_str::<toml::Table>(&fs::read_to_string(&path).unwrap()).unwrap();
                (
                    path_id.clone(),
                    Meta {
                        path: path_id.clone(),
                        tags: match table.remove("tags") {
                            Some(tags @ toml::Value::Array(..)) => tags.try_into().unwrap(),
                            Some(tags) => [tags.try_into().unwrap()].into(),
                            None => IArray::<IString>::EMPTY,
                        }
                        .iter()
                        .map(|tag_id| {
                            tags.get(tag_id.clone()).unwrap_or_else(|| {
                                panic!(
                                    "tag `{tag_id}` used by page meta `{}` {}",
                                    path.display(),
                                    "is not defined in the tags file"
                                )
                            })
                        })
                        .collect(),
                        default_info: Info {
                            title: table
                                .remove("title")
                                .unwrap_or_else(|| {
                                    panic!(
                                        "missing default `title` in page meta {}",
                                        path.display()
                                    )
                                })
                                .try_into()
                                .unwrap(),
                            description: table
                                .remove("description")
                                .unwrap_or_else(|| {
                                    panic!(
                                        "missing default `description` in page meta {}",
                                        path.display()
                                    )
                                })
                                .try_into()
                                .unwrap(),
                        },
                        infos: table
                            .into_iter()
                            .map(|(lang_id, info)| {
                                let lang_id: IString = lang_id.into();
                                let lang = languages.get(lang_id.clone());
                                (
                                    lang.ok().unwrap_or_else(|| {
                                        panic!(
                                            "language `{lang_id}` used by tag `{}` {}",
                                            path.display(), // lol fmt breaks if line too long
                                            "is not defined in the languages file"
                                        )
                                    }),
                                    info.try_into().unwrap(),
                                )
                            })
                            .collect(),
                    },
                )
            })
            .collect(),
    }
}
