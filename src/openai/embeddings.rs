use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::OpenAIClient;

use super::usage::Usage;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmbeddingsOptions {
    pub model: String,
    pub input: Vec<String>,
    pub user: Option<String>,
}

impl CreateEmbeddingsOptions {
    pub fn default(model: &str, input: Vec<String>) -> Self {
        Self {
            model: model.to_owned(),
            input,
            user: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Embedding {
    pub object: String,
    pub embedding: Vec<f64>,
    pub index: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Embeddings {
    pub object: String,
    pub data: Vec<Embedding>,
    pub model: String,
    pub usage: Usage,
}

impl OpenAIClient {
    pub async fn create_embeddings(
        &self,
        opts: &CreateEmbeddingsOptions,
    ) -> Result<Embeddings, Box<dyn Error + Send + Sync>> {
        let uri = self.base_uri.clone() + "/embeddings";
        let api_key = &self.api_key;

        let res = self
            .client
            .post(&uri)
            .header("Authorization", format!("Bearer {api_key}"))
            .json(&opts)
            .send()
            .await?;

        let embeddings: Embeddings = res.json().await?;
        Ok(embeddings)
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
    pub async fn test_create_embeddings() {
        initialize();
        let api_key = env::var("OPENAI_API_KEY").expect("error loading api key");
        let client = OpenAIClient::new(&api_key, "https://api.openai.com/v1");

        let opts = CreateEmbeddingsOptions::default(
            "text-embedding-ada-002",
            vec![
                "I love jenny!".to_owned(),
                "I cant wait to move to LA!".to_owned(),
            ],
        );

        let embeddings = client
            .create_embeddings(&opts)
            .await
            .expect("error creating embeddings");

        println!("{:#?}", embeddings);
    }
}
