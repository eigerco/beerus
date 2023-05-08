# Build the book
echo building book
mdbook build

# Build es and fr versions.
for po_lang in es fr
do
    echo building "$po_lang"
    MDBOOK_BOOK__LANGUAGE="$po_lang" mdbook build -d book/"$po_lang"
done
