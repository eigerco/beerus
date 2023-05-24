# Translations contribution guide

## Introduction

We would love to have your help with translating the Beerus Documentation into other languages! We use the Gettext system for translations. This means that you don't modify the Markdown files directly: instead you modify .po files in a po/ directory. The .po files are small text-based translation databases.
> **Tip**: You should not edit the .po files by hand. Instead use a PO editor, such as Poedit. There are also several online editors available. This will ensure that the file is encoded correct.

## Setup

1. Rust related packages:
   - Install toolchain providing `cargo` using [rustup](https://rustup.rs/).
   - Install [mdBook](https://rust-lang.github.io/mdBook/guide/installation.html) and the translation extension:  
   ```
   cargo install mdbook mdbook-i18n-helpers
   ```
2. Host machine packages:
   - Install [gettext](https://www.gnu.org/software/gettext/) for translations, usually available with regular package manager:  
   `sudo apt install gettext`.
   
3. Clone this repository.

## Work locally (english, main language)

All the Markdown files **MUST** be edited in english. To work locally in english:

   - Start a local server with `mdbook serve` and visit [localhost:3000](http://localhost:3000) to view the book.
   You can use the `--open` flag to open the browser automatically: `mdbook serve --open`.
    
   - Make changes to the book and refresh the browser to see the changes.
    
   - Open a PR with your changes.

## Work locally (translations)

This book is targetting international audience, and aims at being gradually translated in several languages.

**All files in the `src` directory MUST be written in english**. This ensures that all the translation files can be
auto-generated and updated by translators.

To work with translations, those are the steps to update the translated content:
   
   - Run a local server for the language you want to edit: `./translations.sh es` for instance. If no language is provided, the script will only extract translations from english.

   - Open the translation file you are interested in `po/es.po` for instance. You can also use editors like [poedit](https://poedit.net/) to help you on this task.

   - When you are done, you should only have changes into the `po/xx.po` file. Commit them and open a PR.
   The PR must stars with `i18n` to let the maintainers know that the PR is only changing translation.

The translation work is inspired from [Comprehensive Rust repository](https://github.com/google/comprehensive-rust/blob/main/TRANSLATIONS.md).

