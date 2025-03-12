use std::cell::RefCell;
use std::fs::File;
use std::io;
use std::io::Write;

use comrak::html::format_document_with_formatter;
use comrak::html::format_node_default;
use comrak::html::ChildRendering;
use comrak::html::Context;
use comrak::nodes::AstNode;
use comrak::nodes::NodeCode;
use comrak::nodes::NodeCodeBlock;
use comrak::nodes::NodeHeading;
use comrak::nodes::NodeLink;
use comrak::nodes::NodeValue;
use comrak::parse_document;
use comrak::Arena;
use comrak::Options;
use comrak::Plugins;
use implicit_clone::sync::IArray;
use implicit_clone::sync::IString;
use implicit_clone::ImplicitClone;
use itertools::Itertools;

use crate::i18n::I18nStore;
use crate::language::Language;
use crate::meta::MetaStore;
use crate::page::PageStore;
use crate::utils::sync::path::IPath;
use crate::utils::sync::path::ToIPath;

#[derive(Debug, Clone)]
pub struct RenderCtx {
    pub lang: Language,
    pub i18ns: I18nStore,
    pub metas: MetaStore,
    pub pages: PageStore,
}

impl ImplicitClone for RenderCtx {}

thread_local! {
    static RENDER_CONTEXT: RefCell<Option<RenderCtx>> = None.into();
}

pub fn my_render(
    path: IPath,
    content: IString,
    ctx: RenderCtx,
    options: &Options,
    plugins: &Plugins,
) {
    let arena = Arena::new();

    let root = parse_document(&arena, &content, options);

    let mut file = File::create(path.clone()).unwrap();

    RENDER_CONTEXT.with(|r_ctx| *r_ctx.borrow_mut() = Some(ctx));
    format_document_with_formatter(root, options, &mut file, plugins, my_formatter).unwrap();
}

fn my_formatter<'a>(
    context: &mut Context,
    node: &'a AstNode<'a>,
    entering: bool,
) -> io::Result<ChildRendering> {
    let ctx = RENDER_CONTEXT.with(|my_data| my_data.borrow().as_ref().unwrap().clone());

    let borrow = node.data.borrow_mut();
    match borrow.value {
        NodeValue::CodeBlock(NodeCodeBlock { ref info, .. }) if entering => {
            context.write_all(br#"<div class="code">"#)?;

            // header
            context.write_all(b"<div class=\"header\">")?;
            // code
            context.write_all(b"<span>")?;
            context.write_all(info.as_bytes())?;
            context.write_all(b"</span>")?;
            // copy
            context.write_all(b"<span copy>")?;
            context.write_all(
                ctx.i18ns
                    .display("code_copy".into(), ctx.lang.clone())
                    .unwrap()
                    .as_bytes(),
            )?;
            context.write_all(b"</span>")?;
            context.write_all(b"</div>\n")?;

            // content
            drop(borrow);
            format_node_default(context, node, entering)?;

            context.write_all(b"</div>\n")?;
            Ok(ChildRendering::HTML)
        }
        NodeValue::Link(NodeLink { ref url, .. }) if entering => {
            let path_id = url.split('#').next().unwrap().to_ipath();

            let page = ctx.pages.get(path_id.clone());
            let title = node
                .children()
                .next()
                .is_none()
                .then_some(())
                .filter(|()| page.is_some())
                .map(|()| ctx.metas.title(path_id, ctx.lang.clone()).unwrap())
                .unwrap_or_default();

            match page.map(|page| page.keys().cloned().collect::<IArray<_>>()) {
                Some(available_languages) if available_languages.contains(&ctx.lang) => {
                    context.write_all(br#"<a href=""#)?;
                    context.escape_href(format!("/secdb/{}/{}", ctx.lang.id, url).as_bytes())?;
                    context.write_all(br#"">"#)?;
                    context.write_all(title.as_bytes())?;
                    Ok(ChildRendering::HTML)
                }
                Some(_) => {
                    context.write_all(title.as_bytes())?;
                    Ok(ChildRendering::HTML)
                }
                None => {
                    drop(borrow);
                    format_node_default(context, node, entering)
                }
            }
        }
        NodeValue::Link(NodeLink { ref url, .. }) if !entering => {
            match ctx
                .pages
                .get(url.to_ipath())
                .map(|page| page.keys().cloned().collect::<IArray<_>>())
            {
                Some(available_languages) if !available_languages.contains(&ctx.lang) => {
                    for lang in available_languages {
                        context.write_all(br#"<a href=""#)?;
                        context.escape_href(format!("/secdb/{}/{}", lang.id, url).as_bytes())?;
                        context.write_all(br#""><sup>("#)?;
                        context.write_all(lang.id.as_bytes())?;
                        context.write_all(br#")</sup></a>"#)?;
                    }

                    Ok(ChildRendering::HTML)
                }
                _ => {
                    if url.starts_with("https://") || url.starts_with("http://") {
                        writeln!(context, "<sup>(⮥)</sup>")?;
                    }
                    drop(borrow);
                    format_node_default(context, node, entering)
                }
            }
        }
        NodeValue::Heading(NodeHeading { level, .. }) if entering => {
            let fragment = node
                .descendants()
                .filter(|n| !n.same_node(node))
                .flat_map(|node| match node.data.borrow().value {
                    NodeValue::Text(ref text)
                    | NodeValue::Code(NodeCode {
                        literal: ref text, ..
                    }) => text.split(' ').map(|s| s.to_lowercase()).collect(),
                    _ => vec![],
                })
                .filter(|s| !s.is_empty())
                .join("-");

            writeln!(context, r#"<h{level} id="{fragment}">"#)?;
            Ok(ChildRendering::HTML)
        }
        NodeValue::Heading(NodeHeading { level, .. }) if !entering => {
            writeln!(context, r#"<span class="link">🔗</span></h{level}>"#)?;
            Ok(ChildRendering::HTML)
        }
        _ => {
            drop(borrow);
            format_node_default(context, node, entering)
        }
    }
}
