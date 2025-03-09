use std::fs;
use std::ops::Index;

use implicit_clone::sync::IMap;
use implicit_clone::sync::IString;
use implicit_clone::ImplicitClone;

use crate::language::Language;
use crate::language::LanguageStore;
use crate::utils::sync::path::IPath;
use crate::utils::GetRef;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct I18n {
    pub default_display: IString,
    pub displays: IMap<Language, IString>,
}

impl ImplicitClone for I18n {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct I18nStore {
    pub i18ns: IMap<IString, I18n>,
}

impl ImplicitClone for I18nStore {}

impl I18nStore {
    pub fn display(&self, key: IString, lang: Language) -> Option<&IString> {
        self.get(key.clone(), lang)
    }

    pub fn iter_ids(&self) -> impl Iterator<Item = &IString> + '_ {
        self.i18ns.keys()
    }

    fn get(&self, key: IString, lang: Language) -> Option<&IString> {
        self.i18ns.get_ref(&key).map(|i18n| {
            i18n.displays
                .get_ref(&lang)
                .unwrap_or(&i18n.default_display)
        })
    }
}

impl Index<(IString, Language)> for I18nStore {
    type Output = IString;

    fn index(&self, (key, lang): (IString, Language)) -> &IString {
        self.get(key.clone(), lang)
            .unwrap_or_else(|| panic!("requested key {key} not found"))
    }
}

impl Index<(Language, IString)> for I18nStore {
    type Output = IString;

    fn index(&self, (lang, key): (Language, IString)) -> &IString {
        &self[(key, lang)]
    }
}

pub fn process_i18n(i18n_file_path: IPath, languages: LanguageStore) -> I18nStore {
    I18nStore {
        i18ns: toml::from_str::<toml::Table>(&fs::read_to_string(i18n_file_path).unwrap())
            .unwrap()
            .into_iter()
            .map(|(i18n_id, value)| {
                let i18n_id: IString = i18n_id.into();
                let mut table = value.try_into::<toml::Table>().unwrap();
                (
                    i18n_id.clone(),
                    I18n {
                        default_display: table
                            .remove("default")
                            .unwrap_or_else(|| panic!("missing `default` in i18n {i18n_id}"))
                            .try_into()
                            .unwrap(),
                        displays: table
                            .into_iter()
                            .map(|(lang_id, display)| {
                                let lang_id: IString = lang_id.into();
                                (
                                    languages.get(lang_id.clone()).ok().unwrap_or_else(|| {
                                        panic!("invalid language {lang_id} in i18n {i18n_id}")
                                    }),
                                    display.try_into().unwrap(),
                                )
                            })
                            .collect(),
                    },
                )
            })
            .collect(),
    }
}
