use std::error::Error;

use reqwest::Client;

use super::models::{OpenAIGetModelsResponse, OpenAIModel};

pub struct OpenAIClient {
    api_key: String,
    base_uri: String,
    client: Client,
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

    pub async fn get_models(
        &self,
    ) -> Result<OpenAIGetModelsResponse, Box<dyn Error + Send + Sync>> {
        let uri = self.base_uri.clone() + "/models";
        let api_key = self.api_key.clone();
        let res = self
            .client
            .get(&uri)
            .header("Authorization", format!("Bearer {api_key}"))
            .send()
            .await?;
        let models: OpenAIGetModelsResponse = res.json().await?;
        Ok(models)
    }

    pub async fn get_model(
        &self,
        model: &str,
    ) -> Result<OpenAIModel, Box<dyn Error + Send + Sync>> {
        let uri = self.base_uri.clone() + &format!("/models/{model}");
        let api_key = self.api_key.clone();
        let res = self
            .client
            .get(&uri)
            .header("Authorization", format!("Bearer {api_key}"))
            .send()
            .await?;
        let model: OpenAIModel = res.json().await?;
        Ok(model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{env, sync::Once};

    static INIT: Once = Once::new();

    fn initialize() {
        INIT.call_once(|| {
            dotenvy::dotenv().expect("error loading environment variables");
        });
    }

    #[tokio::test]
    async fn test_get_models() {
        initialize();
        let api_key = env::var("OPENAI_API_KEY").expect("error loading API key");
        let client = OpenAIClient::new(&api_key, "https://api.openai.com/v1");
        let models = client.get_models().await.expect("error fetching models");
    }

    #[tokio::test]
    async fn test_get_model() {
        initialize();
    }
}
