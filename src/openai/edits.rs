use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::OpenAIClient;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct EditOptions {
    pub model: String,
    pub input: Option<String>,
    pub instruction: String,
    pub n: Option<u64>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
}

impl OpenAIClient {
    pub async fn create_edit(
        &self,
        opts: &EditOptions,
    ) -> Result<reqwest::Response, Box<dyn Error + Send + Sync>> {
        let uri = self.base_uri.clone() + "/embeddings";
        let api_key = &self.api_key;

        let res = self
            .client
            .post(&uri)
            .header("Authorization", format!("Bearer {api_key}"))
            .json(&opts)
            .send()
            .await?;
        Ok(res)
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
    pub async fn test_create_edit() {}
}
