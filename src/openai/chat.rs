use std::collections::HashMap;
use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::OpenAIClient;

use super::usage::Usage;

#[derive(Debug, Serialize, Deserialize)]
pub enum OpenAIChatRole {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "function")]
    Function,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatMessage {
    role: OpenAIChatRole,
    content: String,
    name: Option<String>,
    function_call: Option<Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatFunction {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Value,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatOptions {
    pub model: String,
    pub messages: Vec<OpenAIChatMessage>,
    pub functions: Option<Vec<OpenAIChatFunction>>,
    pub function_call: Option<Value>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub n: Option<u32>,
    pub stream: Option<bool>,
    pub stop: Option<[String; 4]>,
    pub max_tokens: u64,
    pub presence_penalty: Option<i8>,
    pub frequency_penalty: Option<i8>,
    pub logit_bias: Option<HashMap<String, i8>>,
    pub user: Option<String>,
}

impl OpenAIChatOptions {
    pub fn default(model: &str, messages: Vec<OpenAIChatMessage>, max_tokens: u64) -> Self {
        Self {
            model: model.to_owned(),
            messages,
            functions: None,
            function_call: None,
            temperature: Some(1.0),
            top_p: Some(1.0),
            n: Some(1),
            stream: Some(false),
            stop: None,
            max_tokens,
            presence_penalty: Some(0),
            frequency_penalty: Some(0),
            logit_bias: None,
            user: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatResponseMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatResponseChoice {
    pub index: u64,
    pub message: OpenAIChatResponseMessage,
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatCompletion {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<OpenAIChatResponseChoice>,
    pub usage: Usage,
}

impl OpenAIClient {
    pub async fn get_chat_completion(
        &self,
        opts: &OpenAIChatOptions,
    ) -> Result<OpenAIChatCompletion, Box<dyn Error + Send + Sync>> {
        let uri = self.base_uri.clone() + "/chat/completions";
        let api_key = &self.api_key;
        let res = self
            .client
            .post(&uri)
            .header("Authorization", format!("Bearer {api_key}"))
            .json(&opts)
            .send()
            .await?;
        let completion = res.json().await?;
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
    pub async fn test_chat_completion() {
        initialize();
        let api_key = env::var("OPENAI_API_KEY").expect("error loading API key");
        let client = OpenAIClient::new(&api_key, "https://api.openai.com/v1");
        let x = OpenAIChatMessage {
            role: OpenAIChatRole::System,
            name: None,
            content: "you are a helpful assistant".to_owned(),
            function_call: None,
        };

        println!("{:#?}", x);
        let _completion = client
            .get_chat_completion(&OpenAIChatOptions::default(
                "gpt-3.5-turbo",
                vec![OpenAIChatMessage {
                    role: OpenAIChatRole::System,
                    name: None,
                    content: "you are a helpful assistant".to_owned(),
                    function_call: None,
                }],
                20,
            ))
            .await
            .expect("error fetching chat completion");
        println!("{:#?}", _completion);
    }
}
