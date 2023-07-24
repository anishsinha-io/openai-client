use std::{collections::HashMap, error::Error};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::OpenAIClient;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionOptions {
    pub model: String,
    pub prompt: Vec<String>,
    pub suffix: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub n: Option<u32>,
    pub stream: Option<bool>,
    pub logprobs: Option<u32>,
    pub echo: Option<bool>,
    pub stop: Option<[String; 4]>,
    pub presence_penalty: Option<i8>,
    pub frequency_penalty: Option<i8>,
    pub best_of: Option<u32>,
    pub logit_bias: Option<HashMap<String, i8>>,
    pub user: Option<String>,
}

impl CompletionOptions {
    pub fn default(model: &str, prompt: Vec<String>) -> Self {
        CompletionOptions {
            model: model.to_string(),
            prompt,
            suffix: None,
            max_tokens: Some(16),
            temperature: Some(1.0),
            top_p: Some(1.0),
            n: Some(1),
            stream: Some(false),
            logprobs: None,
            echo: Some(false),
            stop: None,
            presence_penalty: Some(0),
            frequency_penalty: Some(0),
            best_of: Some(1),
            logit_bias: None,
            user: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub text: String,
    pub index: u64,
    pub logprobs: Value,
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAICompletion {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

impl OpenAIClient {
    /// [Completions API](https://platform.openai.com/docs/api-reference/completions/create)
    pub async fn get_completion(
        &self,
        opts: &CompletionOptions,
    ) -> Result<OpenAICompletion, Box<dyn Error + Send + Sync>> {
        let uri = self.base_uri.clone() + "/completions";
        let api_key = &self.api_key;
        let res = self
            .client
            .post(&uri)
            .header("Authorization", format!("Bearer {api_key}"))
            .json(&opts)
            .send()
            .await?;
        let completion: OpenAICompletion = res.json().await?;
        Ok(completion)
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
    async fn test_completion() {
        initialize();
        let api_key = env::var("OPENAI_API_KEY").expect("error loading API key");
        let client = OpenAIClient::new(&api_key, "https://api.openai.com/v1");
        let completion = client
            .get_completion(&CompletionOptions::default(
                "text-davinci-003",
                vec!["Wish me luck on my date with Jenny".to_string()],
            ))
            .await
            .expect("error requesting completion from model");
    }
}
