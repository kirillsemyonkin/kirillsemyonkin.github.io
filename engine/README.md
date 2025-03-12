# engine

Comprised of the following concepts:

- [Languages](src/language.rs) - reading `language.toml` file that looks like following:

    ```toml
    default = "en" # language thought to be default, required
    en = "English" # displays of languages (usually in their respective language)
    ...
    ```

- [I18n](src/i18n.rs) - reading `i18n.toml` file with lang-translation pairs for GUI elements that
                        looks like following:

    ```toml
    [some_key1]                  # key of an i18n
    default = "Some Key 1"       # default translation of i18n (usually in default lang, required)
    en = "Some Key 1 in English" # specific translation of the i18n to some language
    ...
    [some_key2] # all i18ns are put here
    default = "Some Key 2"
    ...
    ```

- [Tags](src/tag.rs) - reading `tags` directory containing files that look like following:

    ```toml
    alt = ["tag1", "tag2"] # alternative ids (optional, string | list of strings)

    title = "My Cool Tag"                        # default title, required
    description = "Short description of the tag" # default description, required

    [en] # translations to some language, optional, lang has to exist
    title = "My Cool Tag in English"
    description = "Short description of the tag in English"
    ...
    ```

- [Metas](src/meta.rs) - reading `.meta.toml` files in `pages` directory containing files that look
                         like following:

    ```toml
    tags = ["tag1", "tag2"] # tags of a page (optional, string | list of strings)

    title = "My Cool Page"                        # default title, required
    description = "Short description of the page" # default description, required

    [en] # translations to some language, optional, lang has to exist
    title = "My Cool Page in English"
    description = "Short description of the page in English"
    ```

- [Pages](src/page.rs) - reading `pages` directory containing `.<lang>.md/html` files. Corresponding
                         metas must exist.

- [Templates](src/template.rs) - reading `templates` directory containing template files.
  
  The used templates are:

  - `layout.md/html` - templates of the whole website layout. Put `<html>` and such here. Content of
                       this template usually the result of other templates.
  - `page.md/html` - template of a single page.
  - `tags.md/html` - template of a all tags page.
  - `tag.md/html` - template of a single tag page.
  
  There are following arguments that can be used in templates:

  - `lang` - current language.
  - `tag` - current tag in case of a tag page.
  - `available_languages` - languages the current page is available in.
  - `pages` - pages of the current tag in case of a tag page.
  - `languages` - all languages.
  - `tags` - all tags.
  - `path` - path id representing the current page (e.g. for `site/en/cat/page` it is `cat/page`).
  - `title` - title of the current page translated to current language.
  - `description` - description of the current page translated to current language.
  - `content` - content of the page in the current language, possibly given by previous template in
                a chain.
