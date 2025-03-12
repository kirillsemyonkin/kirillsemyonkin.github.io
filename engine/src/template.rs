use std::fs;
use std::iter;

use implicit_clone::sync::IArray;
use implicit_clone::sync::IString;
use implicit_clone::ImplicitClone;
use itertools::Itertools;
use serde::Serialize;
use upon::Renderer;
use upon::Template;
use upon::TemplateRef;

use crate::i18n::I18nStore;
use crate::language::Language;
use crate::language::LanguageStore;
use crate::meta::Meta;
use crate::meta::MetaStore;
use crate::tag::Tag;
use crate::tag::TagStore;
use crate::utils::iter_deep;
use crate::utils::path_to_parts_and_first;
use crate::sync::path::IPath;
use crate::sync::path::ToIPath;

pub struct TemplateStore {
    engine: upon::Engine<'static>,
}

trait Render<'render> {
    fn renderer(
        &self,
        engine: &'render upon::Engine<'render>,
        data: impl Serialize,
    ) -> Renderer<'_>;

    fn render(&self, engine: &'render upon::Engine, data: impl Serialize) -> IString {
        self.renderer(engine, data).to_string().unwrap().into()
    }
}

impl<'render> Render<'render> for TemplateRef<'render> {
    fn renderer(&self, _: &'render upon::Engine<'render>, data: impl Serialize) -> Renderer<'_> {
        self.render(data)
    }
}

impl<'render> Render<'render> for Template<'render> {
    fn renderer(
        &self,
        engine: &'render upon::Engine<'render>,
        data: impl Serialize,
    ) -> Renderer<'_> {
        self.render(engine, data)
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct PageMeta {
    pub path: IString,
    pub tags: IArray<IString>,
    pub available_in_lang: bool,
    pub languages: IArray<IString>,
}

impl ImplicitClone for PageMeta {}

#[derive(Debug, Clone)]
pub struct Context {
    pub current_lang: Language,
    pub current_tag: Option<Tag>,
    pub pages: IArray<(Meta, IArray<Language>)>,

    pub languages: LanguageStore,
    pub tags: TagStore,

    pub page: PageMeta,
    pub title: Option<IString>,
    pub description: Option<IString>,
}

impl ImplicitClone for Context {}

impl TemplateStore {
    pub fn compile_and_render(
        &self,
        filepath: IPath,
        context: Context,
        content: Option<IString>,
    ) -> IString {
        self.render_with_template(
            self.engine
                .compile(fs::read_to_string(filepath).unwrap())
                .unwrap(),
            context,
            content,
        )
    }

    // do not forget to add to [engine/README.md] for possible template arguments
    pub fn render(&self, id: IString, context: Context, content: Option<IString>) -> IString {
        match self.engine.get_template(&id) {
            None => content.unwrap_or_else(|| panic!("template `{id}` not found")),
            Some(template) => self.render_with_template(template, context, content),
        }
    }

    fn render_with_template<'render>(
        &'render self,
        template: impl Render<'render> + 'render,
        Context {
            current_lang,
            current_tag,
            pages,

            languages,
            tags,

            page,
            title,
            description,
        }: Context,
        content: Option<IString>,
    ) -> IString {
        template.render(
            &self.engine,
            upon::value! {
                lang: current_lang.id.clone(),
                tag: current_tag.map(|tag| tag.id),
                pages: pages
                    .iter()
                    .map(move |(meta, available_languages)| PageMeta {
                        path: meta.path.into_iter_lossy().join("/").into(),
                        tags: meta.tags.iter().cloned().map(|tag| tag.id).collect(),
                        available_in_lang: available_languages.contains(&current_lang),
                        languages: available_languages
                            .iter()
                            .map(|lang| lang.id.clone())
                            .collect(),
                    })
                    .collect::<IArray<_>>(),

                languages: languages
                    .iter_ids()
                    .cloned()
                    .collect::<IArray<_>>(),
                tags: tags.iter().map(|tag| tag.id.clone()).unique().collect::<IArray<_>>(),
                default_lang: languages.default.id,

                page: page,
                title: title,
                description: description,
                content: content,
            },
        )
    }
}

pub fn process_templates(
    template_dir: IPath,
    languages: LanguageStore,
    i18n: I18nStore,
    tags: TagStore,
    metas: MetaStore,
) -> TemplateStore {
    let mut engine = upon::Engine::new();

    engine.add_filter("eq", |a: String, b: String| a == b);
    engine.add_filter("lang_display", {
        let languages = languages.clone();
        move |lang_id: String| languages.get(lang_id.into()).unwrap().display.to_string()
    });
    engine.add_filter("tag_title", {
        let tags = tags.clone();
        let languages = languages.clone();
        move |tag_id: String, lang_id: String| {
            tags.get(tag_id.into())
                .unwrap()
                .title(languages.get(lang_id.into()).unwrap())
                .to_string()
        }
    });
    engine.add_filter("tag_description", {
        let tags = tags.clone();
        let languages = languages.clone();
        move |tag_id: String, lang_id: String| {
            tags.get(tag_id.into())
                .unwrap()
                .description(languages.get(lang_id.into()).unwrap())
                .to_string()
        }
    });
    engine.add_filter("i18n", {
        let languages = languages.clone();
        let i18n = i18n.clone();
        move |i18n_id: String, lang_id: String| {
            let i18n_id: IString = i18n_id.into();
            i18n.display(i18n_id.clone(), languages.get(lang_id.into()).unwrap())
                .unwrap_or_else(|| panic!("missing i18n {i18n_id}"))
                .to_string()
        }
    });
    engine.add_filter("page_title", {
        let metas = metas.clone();
        let languages = languages.clone();
        move |path_id: String, lang_id: String| {
            metas
                .get(path_id.to_ipath())
                .unwrap()
                .title(languages.get(lang_id.into()).unwrap())
                .to_string()
        }
    });
    engine.add_filter("page_description", {
        let metas = metas.clone();
        let languages = languages.clone();
        move |path_id: String, lang_id: String| {
            metas
                .get(path_id.to_ipath())
                .unwrap()
                .description(languages.get(lang_id.into()).unwrap())
                .to_string()
        }
    });
    engine.add_filter("subpaths", {
        let languages = languages.clone();
        let i18n = i18n.clone();
        move |path_id: String, lang_id: String| {
            let lang = languages.get(lang_id.into()).unwrap();

            let mut parts = path_id.split('/').collect_vec();
            if parts.first() != Some(&"") {
                parts.insert(0, "");
            }

            (0..parts.len())
                .map(|i| {
                    let name = match &parts[1..=i] {
                        ["tags", tag_id] => tags[(*tag_id).into()].title(lang.clone()),
                        ["tags"] => i18n
                            .display("all_tags".into(), lang.clone())
                            .unwrap()
                            .clone(),
                        page_id => metas[page_id.iter().collect()].title(lang.clone()),
                    };
                    upon::value! {
                        path: parts[1..=i].join("/"),
                        name: name,
                    }
                })
                .collect_vec()
        }
    });

    for path in iter_deep(template_dir.clone()) {
        let path_id = path.strip_prefix(template_dir.clone()).unwrap();
        let (path_parts, first_part) = path_to_parts_and_first(path_id);
        let reassembled_path = path_parts
            .into_iter()
            .chain(iter::once(first_part))
            .join("/");
        engine
            .add_template(reassembled_path, fs::read_to_string(path).unwrap())
            .unwrap();
    }

    TemplateStore { engine }
}
