# Refresh the messages.pot, basis for all translations.
MDBOOK_OUTPUT='{"xgettext": {"pot-file": "messages.pot"}}' \
mdbook build -d po

# Merge the translations for each languages.
for po_lang in es fr
do
    msgmerge --update po/"$po_lang".po po/messages.pot
done
