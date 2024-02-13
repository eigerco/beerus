# How to deploy and work with this little example

## Setup

To build the current repo content, several packages are required.

1. Rust related packages:
    * cargo must be installed (install [rustup](https://rustup.rs/) if not already done).
    * install the following package with cargo: `cargo install mdbook mdbook-i18n-helpers`
2. Host packages:
    * gettext package for translations: `sudo apt install gettext`

## Build

To build the book, just use `mdbook build` being at the root of this repository.
This will generate the content into `./book` folder.

The most important file to build is the `SUMMARY.md`, which is giving `mdbook` the information
of which files must be included.

Also, all the information related to the extra files to be included can be found in the
`book.toml` configuration file.

## Translations

This book is targeting international audience, and we inspired our work from [Google repos](https://github.com/google/comprehensive-rust).
Please refer to [this page](https://github.com/google/comprehensive-rust/blob/5bbb68be2cee0f2ee1b5be96c97e5a6aad385b1f/TRANSLATIONS.md) for the original documentation. In this documentation details can be found to build translations and add new ones.

Basically, each translation can be found in a `.po` file where the translators only have to translate chunk of texts
extracted by the `gettext` tools.

**All files in the `src` directory MUST be written in english**. Then all the translation files can be
generated and updated with few commands.

When english text is changed, those are the step to merge the new content:

1. Rebuild the `messages.pot`. This must be done ANYTIME the english version is changed.
   `MDBOOK_OUTPUT='{"xgettext": {"pot-file": "messages.pot"}}' mdbook build -d po`
   
2. To add a new translation, just run the following command replacing `xx` by the two letters of the language.
   `msginit -i po/messages.pot -l xx -o po/xx.po`

2. Merge the changes if the translation was already generated, but english version was rebuild.
   In this process, unchanged messages are intact, deleted message are marked as old and can be removed, and the updated messaged
   marked as fuzzy and must be updated before removing the marker.
   `msgmerge --update po/xx.po po/messages.pot`

   Fuzzy marker looks like this:
   ```
   #: src/page1.md:3
   #, fuzzy
   msgid "A blob of test being translated.CHANGED"
   msgstr "Un bout de text à traduire."
   ```

   If the fuzzy marker is not removed, the text will be displayed in english instead of
   the translation (even if `msgstr` is set, as shown in this example. Only removing the
   fuzzy marker will render in the target language.

## Test locally

1. Merge the translations with `sh translate.sh` to ensure no updated are needed. Edit all required stuff in the `po` dir.

2. Build the book and all translations `sh build.sh`.

3. Open the book with a browser `firefox book/index.html`.
