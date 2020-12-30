require("dotenv-safe").config();

const MeiliSearch = require("meilisearch");
const json = require("./gerbode.json");

const config = {
  host: process.env.HOST,
  apiKey: process.env.MEILI_API_KEY,
};

const client = new MeiliSearch(config);

const indexName = "gerbode";

const chunk = function (array, size) {
  if (!array.length) {
    return [];
  }
  const head = array.slice(0, size);
  const tail = array.slice(size);

  return [head, ...chunk(tail, size)];
};

const json_array = chunk(json, (json.length / 10) | 1);

async function createIndex() {
  const result = await client.createIndex({ uid: indexName, primaryKey: "id" });
  console.log(result);
}

async function addDocuments() {
  console.log(
    "Preparing to upload",
    json.length,
    "pieces in",
    json_array.length,
    "chunks"
  );
  const index = client.getIndex(indexName);
  for await (const json of json_array) {
    console.log("Inserting chunk of length", json.length);
    const result = await index.addDocuments(json);
    console.log("Chunk complete", result);
  }
}

async function query() {
  const index = client.getIndex(indexName);
  const search = await index.search("Dowland");
  console.log(search);
}

async function deleteIndex() {
  const index = client.getIndex(indexName);
  await index.deleteIndex();
  console.log(indexName, "index deleted");
}

async function getStats() {
  const stats = await client.databaseStats();
  console.log(stats);
}

async function updateSettings() {
  console.log("Updating settings");
  const index = client.getIndex(indexName);
  await index.updateSettings({
    searchableAttributes: [
      "composer",
      "title",
      "subtitle",
      "date",
      "type_of_piece",
      "document",
      "original_composer",
      "source",
      "difficulty",
      "key",
      "ensemble",
      "part",
      "volume",
      "concordances",
      "piece",
      "section",
      "remarks",
      "editor",
      "encoder",
      "arranger",
      "intabulator",
      "contributor",
      "page",
      "is_date_approximate",
    ],
  });
}

(async function main() {
  try {
    await deleteIndex();
    await createIndex();
    await addDocuments();
    await updateSettings();
  } catch (e) {
    /* handle error */
    console.log(e.message);
  }
})();
