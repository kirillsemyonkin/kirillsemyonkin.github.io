#![allow(clippy::duplicate_mod)]

pub mod i18n;
pub mod language;
pub mod meta;
pub mod page;
pub mod render;
pub mod sync;
pub mod tag;
pub mod template;
pub mod unsync;
pub mod utils;

use std::fs;
use std::io::Cursor;

use comrak::plugins::syntect::SyntectAdapterBuilder;
use comrak::ExtensionOptions;
use comrak::Options;
use comrak::Plugins;
use comrak::RenderOptions;
use comrak::RenderPlugins;
use implicit_clone::sync::IArray;
use implicit_clone::sync::IString;
use itertools::Itertools;

use crate::i18n::process_i18n;
use crate::language::process_languages;
use crate::meta::process_metas;
use crate::page::process_pages;
use crate::render::my_render;
use crate::render::RenderCtx;
use crate::sync::path::IPath;
use crate::tag::process_tags;
use crate::template::process_templates;
use crate::template::Context;
use crate::template::PageMeta;

fn main() {
    run(IPath::new("secdb"), IPath::new("public/secdb"));
}

fn run(src_dir_path: IPath, public_dir_path: IPath) {
    let languages = process_languages(src_dir_path.join("languages.toml"));
    println!(
        "languages: [{}]",
        languages.iter_ids().map(|id| format!("`{id}`")).join(", ")
    );

    let i18ns = process_i18n(src_dir_path.join("i18n.toml"), languages.clone());
    println!(
        "i18ns: [{}]",
        i18ns.iter_ids().map(|id| format!("`{id}`")).join(", ")
    );

    let tags = process_tags(src_dir_path.join("tags"), languages.clone());
    println!(
        "tags: [{}]",
        tags.iter_ids().map(|id| format!("`{id}`")).join(", ")
    );

    let pages_dir_path = src_dir_path.join("pages");
    let metas = process_metas(pages_dir_path.clone(), languages.clone(), tags.clone());
    println!(
        "metas: [{}]",
        metas
            .iter_ids()
            .map(|id| format!("`{}`", id.display()))
            .join(", ")
    );

    let pages = process_pages(pages_dir_path.clone(), languages.clone(), metas.clone());
    let templates = process_templates(
        src_dir_path.join("templates"),
        languages.clone(),
        i18ns.clone(),
        tags.clone(),
        metas.clone(),
    );
    // do not forget to update [engine/README.md] for used templates

    fs::create_dir_all(&public_dir_path).unwrap();
    let options = Options {
        extension: ExtensionOptions {
            header_ids: Some("".to_string()),
            ..Default::default()
        },
        render: RenderOptions {
            unsafe_: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let highlighter = SyntectAdapterBuilder::new()
        .theme_set(syntect::highlighting::ThemeSet {
            themes: [(
                "darcula".into(),
                syntect::highlighting::ThemeSet::load_from_reader(&mut Cursor::new(include_str!(
                    "../../public/assets/darcula.tmTheme"
                )))
                .unwrap(),
            )]
            .into(),
        })
        .theme("darcula")
        .build();
    let plugins = Plugins {
        render: RenderPlugins {
            codefence_syntax_highlighter: Some(&highlighter),
            ..Default::default()
        },
    };

    // write all pages
    for (path_id, page_lang_path_map) in pages.iter() {
        let meta = metas
            .get(path_id.clone())
            .unwrap_or_else(|| panic!("missing page meta for page `{}`", path_id.display()));
        let path_id: IString = path_id.into_iter_lossy().join("/").into();

        let available_languages = page_lang_path_map
            .keys()
            .map(|lang| lang.id.clone())
            .collect::<IArray<_>>();

        for (lang, index_filepath) in page_lang_path_map.iter() {
            let info = meta.info(lang.clone());
            let context = Context {
                current_lang: lang.clone(),
                current_tag: None,
                pages: Default::default(),

                languages: languages.clone(),
                tags: tags.clone(),

                page: PageMeta {
                    path: path_id.clone(),
                    tags: meta.tags.iter().map(|tag| tag.id.clone()).collect(),
                    available_in_lang: true,
                    languages: available_languages.clone(),
                },
                title: Some(info.title.clone()),
                description: Some(info.description.clone()),
            };
            let content =
                templates.compile_and_render(index_filepath.clone(), context.clone(), None);
            let content = templates.render("page".into(), context.clone(), Some(content));
            let content = templates.render("layout".into(), context, Some(content));

            let path = public_dir_path
                .join(&*lang.id)
                .join(&*path_id)
                .join("index.html");
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            my_render(
                path,
                content,
                RenderCtx {
                    lang: lang.clone(),
                    i18ns: i18ns.clone(),
                    metas: metas.clone(),
                    pages: pages.clone(),
                },
                &options,
                &plugins,
            );
        }
    }

    // write the "all tags" pages
    for lang in languages.iter() {
        let context = Context {
            current_lang: lang.clone(),
            current_tag: None,
            pages: Default::default(),

            languages: languages.clone(),
            tags: tags.clone(),

            page: PageMeta {
                path: "tags".into(),
                tags: Default::default(),
                available_in_lang: true,
                languages: languages.iter_ids().cloned().collect(),
            },
            title: Some(
                i18ns
                    .display("all_tags".into(), lang.clone())
                    .unwrap_or_else(|| panic!("missing i18n for all_tags"))
                    .clone(),
            ),
            description: None,
        };
        let content = templates.render("tags".into(), context.clone(), None);
        let content = templates.render("layout".into(), context, Some(content));
        let path = public_dir_path.join(&*lang.id).join("tags/index.html");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        my_render(
            path,
            content,
            RenderCtx {
                lang: lang.clone(),
                i18ns: i18ns.clone(),
                metas: metas.clone(),
                pages: pages.clone(),
            },
            &options,
            &plugins,
        );
    }

    // write all tag pages
    for lang in languages.iter() {
        for tag_id in tags.iter_ids().cloned() {
            let tag = tags.get(tag_id.clone()).unwrap();
            let content = match tag.id == tag_id {
                true => {
                    let context = Context {
                        current_lang: lang.clone(),
                        current_tag: Some(tag.clone()),
                        pages: metas
                            .iter_by_tag(tag.clone())
                            .map(|meta| {
                                let available_langs = pages
                                    .get(meta.path.clone())
                                    .unwrap()
                                    .keys()
                                    .cloned()
                                    .collect();
                                (meta.clone(), available_langs)
                            })
                            .collect(),

                        languages: languages.clone(),
                        tags: tags.clone(),

                        page: PageMeta {
                            path: format!("tags/{tag_id}").into(),
                            tags: Default::default(),
                            available_in_lang: true,
                            languages: languages.iter_ids().cloned().collect(),
                        },
                        title: Some(tag.title(lang.clone())),
                        description: Some(tag.description(lang.clone())),
                    };
                    let content = templates.render("tag".into(), context.clone(), None);
                    templates.render("layout".into(), context, Some(content))
                }
                false => format!(
                    "<meta http-equiv=\"refresh\" content=\"0; url=/{}/tags/\">",
                    lang.id
                )
                .into(),
            };
            let path = public_dir_path
                .join(&*lang.id)
                .join("tags")
                .join(&*tag_id)
                .join("index.html");
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            my_render(
                path,
                content,
                RenderCtx {
                    lang: lang.clone(),
                    i18ns: i18ns.clone(),
                    metas: metas.clone(),
                    pages: pages.clone(),
                },
                &options,
                &plugins,
            );
        }
    }
}
