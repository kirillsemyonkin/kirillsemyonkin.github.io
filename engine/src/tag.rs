use std::fs;
use std::iter;

use implicit_clone::sync::IArray;
use implicit_clone::sync::IMap;
use implicit_clone::sync::IString;
use implicit_clone::ImplicitClone;

use crate::language::Language;
use crate::language::LanguageStore;
use crate::utils::iter_deep;
use crate::utils::path_to_parts_and_first;
use crate::utils::sync::path::IPath;
use crate::utils::Info;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    pub id: IString,
    pub alt_ids: IArray<IString>,
    pub default_info: Info<IString>,
    pub infos: IMap<Language, Info<Option<IString>>>,
}

impl Tag {
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
}

impl ImplicitClone for Tag {}

#[derive(Debug, Clone)]
pub struct TagStore {
    pub tags: IMap<IString, Tag>,
}

impl TagStore {
    pub fn get(&self, id: IString) -> Option<Tag> {
        self.tags.get(&id)
    }

    pub fn title(&self, id: IString, lang: Language) -> Option<IString> {
        self.get(id).map(|tag| tag.title(lang))
    }

    pub fn description(&self, id: IString, lang: Language) -> Option<IString> {
        self.get(id).map(|tag| tag.description(lang))
    }

    pub fn info(&self, id: IString, lang: Language) -> Option<Info<IString>> {
        self.get(id).map(|tag| tag.info(lang))
    }

    pub fn iter(&self) -> impl Iterator<Item = &Tag> + '_ {
        self.tags.values()
    }

    pub fn iter_ids(&self) -> impl Iterator<Item = &IString> + '_ {
        self.tags.keys()
    }
}

impl ImplicitClone for TagStore {}

pub fn process_tags(tags_dir_path: IPath, languages: LanguageStore) -> TagStore {
    TagStore {
        tags: iter_deep(tags_dir_path)
            .map(|path| {
                let mut table =
                    toml::from_str::<toml::Table>(&fs::read_to_string(&path).unwrap()).unwrap();
                let (_, id) = path_to_parts_and_first(path);
                Tag {
                    id: id.clone(),
                    alt_ids: match table.remove("alt") {
                        Some(value @ toml::Value::Array(..)) => value.try_into().unwrap(),
                        Some(value) => [value.try_into().unwrap()].into(),
                        None => IArray::EMPTY,
                    },
                    default_info: Info {
                        title: table
                            .remove("title")
                            .unwrap_or_else(|| panic!("missing default `title` in tag {id}"))
                            .try_into()
                            .unwrap(),
                        description: table
                            .remove("description")
                            .unwrap_or_else(|| panic!("missing default `description` in tag {id}"))
                            .try_into()
                            .unwrap(),
                    },
                    infos: table
                        .into_iter()
                        .map(|(lang_id, info)| {
                            let lang_id: IString = lang_id.into();
                            let lang = languages.get(lang_id.clone());
                            assert!(
                                lang.is_ok(),
                                "{lang_id} used by tag {id} is not defined in the languages file"
                            );
                            (lang.unwrap(), info.try_into().unwrap())
                        })
                        .collect(),
                }
            })
            .flat_map(|tag| {
                iter::once(tag.id.clone())
                    .chain(tag.alt_ids.clone())
                    .map(move |id| (id, tag.clone()))
            })
            .collect(),
    }
}
