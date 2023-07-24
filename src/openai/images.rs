use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::OpenAIClient;

#[derive(Debug, Serialize, Deserialize)]
pub enum ImgSize {
    #[serde(rename = "256x256")]
    Size256x265,
    #[serde(rename = "512x512")]
    Size512x512,
    #[serde(rename = "1024x1024")]
    Size1024x1024,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ImgFormat {
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "b64_json")]
    Base64Json,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateImgOptions {
    pub prompt: String,
    pub n: Option<u8>,
    pub size: Option<ImgSize>,
    pub response_format: Option<ImgFormat>,
    pub user: Option<String>,
}

impl CreateImgOptions {
    pub fn default(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_owned(),
            n: Some(1),
            size: Some(ImgSize::Size256x265),
            response_format: Some(ImgFormat::Url),
            user: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Img {
    #[serde(alias = "url", alias = "b64_json")]
    img_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateImgResponse {
    created: u64,
    data: Vec<Img>,
}

impl OpenAIClient {
    pub async fn create_image_url(
        &self,
        opts: &CreateImgOptions,
    ) -> Result<CreateImgResponse, Box<dyn Error + Send + Sync>> {
        let uri = self.base_uri.clone() + "/images/generations";
        let api_key = &self.api_key;
        let res = self
            .client
            .post(&uri)
            .header("Authorization", format!("Bearer {api_key}"))
            .json(&opts)
            .send()
            .await?;
        let images: CreateImgResponse = res.json().await?;
        Ok(images)
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
    pub async fn test_create_image_url() {
        initialize();
        let api_key = env::var("OPENAI_API_KEY").expect("error loading API key");
        let client = OpenAIClient::new(&api_key, "https://api.openai.com/v1");
        let mut opts = CreateImgOptions::default(
            "A pretty house in Beverly Hills, California where Jenny and I can move to eventually",
        );
        // let opts = CreateImgOptions::default(
        //     "A pretty house in Beverly Hills, California where Jenny and I can move to eventually",
        // );
        opts.response_format = Some(ImgFormat::Base64Json);
        let _images = client
            .create_image_url(&opts)
            .await
            .expect("error creating image");
    }
}
