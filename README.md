# Gerbode MeiliSearch index

This repo parses a spreadsheet containing lute music data and uploads the it to a MeiliSearch service.

The search service is then used as the backend for my [lute-search](https://keliris.dev/lute-search) project.

## Update search index

-
- Run the script to send the json up to the MeiliSearch server `node index.js`

```sh
# Download the latest spreadsheet
curl https://www.lutemusic.org/spreadsheet.xlsx > spreadsheet.xlsx

# Parse the spreadsheet (with some modifications) into a .csv and add an ID based on the index
cargo run --release

# Upload the .csv to the MeiliSearch instance (.csv is more efficient)
curl \
  -X POST '$HOST/indexes/lute/documents?primaryKey=id' \
  -H 'Content-Type: text/csv' \
  -H 'Authorization: Bearer test' \
  --data-binary @gerbode.csv
```
