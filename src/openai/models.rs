use crate::OpenAIClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIModelPermission {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub allow_create_engine: bool,
    pub allow_sampling: bool,
    pub allow_logprobs: bool,
    pub allow_search_indices: bool,
    pub allow_view: bool,
    pub allow_fine_tuning: bool,
    pub organization: String,
    pub group: Value,
    pub is_blocking: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIModel {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub owned_by: String,
    pub permission: Vec<OpenAIModelPermission>,
    pub root: String,
    pub parent: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIGetModelsResponse {
    pub object: String,
    pub data: Vec<OpenAIModel>,
}

impl OpenAIClient {
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
        let _models = client.get_models().await.expect("error fetching models");
    }

    #[tokio::test]
    async fn test_get_model() {
        initialize();
        let api_key = env::var("OPENAI_API_KEY").expect("error loading API key");
        let client = OpenAIClient::new(&api_key, "https://api.openai.com/v1");
        let _model = client
            .get_model("text-davinci-003")
            .await
            .expect("error fetching model");
    }
}
