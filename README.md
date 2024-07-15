# Gerbode MeiliSearch index

This repo parses a spreadsheet containing lute music data and uploads it to a MeiliSearch service.

The search service is then used as the backend for my [lute-search](https://keliris.dev/lute-search) project.

## Update search index

```
cargo run --release
```

This will fetch the spreadsheet, parse it, upload to MeiliSearch and configure the index settings.

Update your local `.env` if you want to seed a local MeiliSearch instance.
