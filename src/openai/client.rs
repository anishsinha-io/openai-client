use reqwest::Client;

pub struct OpenAIClient {
    pub api_key: String,
    pub base_uri: String,
    pub client: Client,
}

impl OpenAIClient {
    pub fn new(api_key: &str, base_uri: &str) -> Self {
        // let trimmed_uri = &base_uri[..base_uri.len() - 1];

        OpenAIClient {
            api_key: api_key.to_string(),
            base_uri: base_uri.to_string(),
            client: Client::new(),
        }
    }
}
