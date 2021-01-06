# Gerbode MeiliSearch index

This repo parses a spreadsheet containing lute music data and uploads the it to a MeiliSearch service.

The search service is then used as the backend for my [lute-search](https://keliris.dev/lute-search) project.

## Update search index

- Download the latest spreadsheet `curl https://www.lutemusic.org/spreadsheet.xlsx > spreadsheet.xlsx`
- Parse the spreadsheet (with some modifications) into json `cargo run --release`
- Run the script to send the json up to the MeiliSearch server `node index.js`
