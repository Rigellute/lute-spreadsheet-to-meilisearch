use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub meili_key: String,
    pub meili_host: String,
    pub lute_music_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        let meili_key =
            env::var("MEILISEARCH_API_KEY").expect("env var MEILISEARCH_API_KEY is required");
        let meili_host =
            env::var("MEILISEARCH_HOST").expect("env var MEILISEARCH_HOST is required");
        let lute_music_url =
            env::var("LUTE_MUSIC_URL").expect("env var LUTE_MUSIC_URL is required");

        Config {
            meili_key,
            meili_host,
            lute_music_url,
        }
    }
}
