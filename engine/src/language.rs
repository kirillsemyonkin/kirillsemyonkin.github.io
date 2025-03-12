use std::fs;

use implicit_clone::sync::IMap;
use implicit_clone::sync::IMapValues;
use implicit_clone::sync::IString;
use implicit_clone::ImplicitClone;

use crate::sync::path::IPath;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Language {
    pub id: IString,
    pub display: IString,
}

impl ImplicitClone for Language {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageStore {
    pub default: Language,
    pub languages: IMap<IString, Language>,
}

impl LanguageStore {
    pub fn get(&self, id: IString) -> Result<Language, Language> {
        self.languages.get(&id).ok_or_else(|| self.default.clone())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Language> + '_ {
        self.languages.values()
    }

    pub fn iter_ids(&self) -> impl Iterator<Item = &IString> + '_ {
        self.languages.keys()
    }
}

impl ImplicitClone for LanguageStore {}

impl<'a> IntoIterator for &'a LanguageStore {
    type Item = Language;

    type IntoIter = std::iter::Cloned<IMapValues<'a, IString, Language>>;

    fn into_iter(self) -> Self::IntoIter {
        self.languages.values().cloned()
    }
}

pub fn process_languages(languages_file_path: IPath) -> LanguageStore {
    let mut table =
        toml::from_str::<toml::Table>(&fs::read_to_string(languages_file_path).unwrap()).unwrap();
    let default_id: IString = table
        .remove("default")
        .expect("missing `default` language setting in languages file")
        .try_into()
        .unwrap();
    let languages = table
        .into_iter()
        .map(|(id, display)| {
            let id: IString = id.into();
            (
                id.clone(),
                Language {
                    id,
                    display: display.try_into().unwrap(),
                },
            )
        })
        .collect::<IMap<_, _>>();
    LanguageStore {
        default: languages
            .get(&default_id)
            .expect("`default` language setting in the languages file is invalid")
            .clone(),
        languages,
    }
}
