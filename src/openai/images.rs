use core::fmt;
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Display};

use crate::OpenAIClient;

#[derive(Debug, Serialize, Deserialize)]
pub enum ImgSize {
    #[serde(rename = "256x256")]
    Size256x256,
    #[serde(rename = "512x512")]
    Size512x512,
    #[serde(rename = "1024x1024")]
    Size1024x1024,
}

impl Display for ImgSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        match self {
            ImgSize::Size256x256 => write!(f, "256x256"),
            ImgSize::Size512x512 => write!(f, "512x512"),
            ImgSize::Size1024x1024 => write!(f, "1024x1024"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ImgFormat {
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "b64_json")]
    Base64Json,
}

/// OpenAI only supports sending images as PNGs. This enum will be updated if/when they update
/// their API
#[derive(Debug, Serialize, Deserialize)]
pub enum ImgType {
    Png,
}

impl Display for ImgFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImgFormat::Url => write!(f, "url"),
            ImgFormat::Base64Json => write!(f, "b64_json"),
        }
    }
}

impl Display for ImgType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImgType::Png => write!(f, "image/png"),
        }
    }
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct EditImgOptions {
    pub file_name: String,
    pub img: Vec<u8>,
    pub prompt: String,
    pub mask: Option<Vec<u8>>,
    pub n: Option<u8>,
    pub size: Option<ImgSize>,
    pub response_format: Option<ImgFormat>,
    pub user: Option<String>,
    pub img_type: ImgType,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateImgVariationsOptions {
    pub file_name: String,
    pub img: Vec<u8>,
    pub n: Option<u8>,
    pub size: Option<ImgSize>,
    pub response_format: Option<ImgFormat>,
    pub user: Option<String>,
    pub img_type: ImgType,
}

impl CreateImgOptions {
    pub fn default(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_owned(),
            n: Some(1),
            size: Some(ImgSize::Size256x256),
            response_format: Some(ImgFormat::Url),
            user: None,
        }
    }
}

impl EditImgOptions {
    pub fn default(file_name: &str, img: Vec<u8>, img_type: ImgType, prompt: &str) -> Self {
        Self {
            file_name: file_name.to_owned(),
            img,
            prompt: prompt.to_owned(),
            mask: None,
            n: Some(1),
            size: Some(ImgSize::Size256x256),
            response_format: Some(ImgFormat::Url),
            user: None,
            img_type,
        }
    }
}

impl CreateImgVariationsOptions {
    pub fn default(file_name: &str, img: Vec<u8>, img_type: ImgType) -> Self {
        Self {
            file_name: file_name.to_owned(),
            img,
            n: Some(1),
            size: Some(ImgSize::Size256x256),
            response_format: Some(ImgFormat::Url),
            user: None,
            img_type,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Img {
    #[serde(alias = "url", alias = "b64_json")]
    img_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImgResponse {
    created: u64,
    data: Vec<Img>,
}

impl OpenAIClient {
    pub async fn create_image(
        &self,
        opts: &CreateImgOptions,
    ) -> Result<ImgResponse, Box<dyn Error + Send + Sync>> {
        let uri = self.base_uri.clone() + "/images/generations";
        let api_key = &self.api_key;
        let res = self
            .client
            .post(&uri)
            .header("Authorization", format!("Bearer {api_key}"))
            .json(&opts)
            .send()
            .await?;
        let images: ImgResponse = res.json().await?;
        Ok(images)
    }

    pub async fn edit_img(
        &self,
        opts: &EditImgOptions,
    ) -> Result<ImgResponse, Box<dyn Error + Send + Sync>> {
        let uri = self.base_uri.clone() + "/images/edits";
        let api_key = &self.api_key;

        let mut form_data = multipart::Form::new();

        let img = multipart::Part::bytes(opts.img.clone())
            .file_name(opts.file_name.to_owned())
            .mime_str(&opts.img_type.to_string())?;

        let prompt = multipart::Part::bytes(opts.prompt.as_bytes().to_owned());

        form_data = form_data
            .part("image".to_owned(), img)
            .part("prompt".to_owned(), prompt);

        if let Some(n) = opts.n {
            // let n_bytes = multipart::Part::bytes(n.to_le_bytes().to_vec());
            // form_data = form_data.part("n".to_owned(), n_bytes)
            form_data = form_data.text::<String, String>("n".to_owned(), n.to_string());
        };

        if let Some(mask) = &opts.mask {
            let mask_bytes = multipart::Part::bytes(mask.to_owned());
            form_data = form_data.part("mask".to_owned(), mask_bytes)
        }

        if let Some(size) = &opts.size {
            let size_bytes = multipart::Part::bytes(size.to_string().as_bytes().to_owned());
            form_data = form_data.part("size", size_bytes);
        }

        if let Some(format) = &opts.response_format {
            let fmt_bytes = multipart::Part::bytes(format.to_string().as_bytes().to_owned());
            form_data = form_data.part("response_format", fmt_bytes);
        }

        if let Some(user) = &opts.user {
            let user_bytes = multipart::Part::bytes(user.as_bytes().to_owned());
            form_data = form_data.part("user", user_bytes);
        }

        let res = self
            .client
            .post(&uri)
            .header("Authorization", format!("Bearer {api_key}"))
            .multipart(form_data)
            .send()
            .await?;

        let images: ImgResponse = res.json().await?;
        Ok(images)
    }

    pub async fn create_img_variations(
        &self,
        opts: &CreateImgVariationsOptions,
    ) -> Result<ImgResponse, Box<dyn Error + Send + Sync>> {
        let uri = self.base_uri.clone() + "/images/variations";
        let api_key = &self.api_key;

        let mut form_data = multipart::Form::new();

        let img = multipart::Part::bytes(opts.img.clone())
            .file_name(opts.file_name.to_owned())
            .mime_str(&opts.img_type.to_string())?;

        form_data = form_data.part("image", img);

        if let Some(n) = opts.n {
            form_data = form_data.text("n", n.to_string());
        };

        if let Some(size) = &opts.size {
            let size_bytes = multipart::Part::bytes(size.to_string().as_bytes().to_owned());
            form_data = form_data.part("size", size_bytes);
        }

        if let Some(format) = &opts.response_format {
            let fmt_bytes = multipart::Part::bytes(format.to_string().as_bytes().to_owned());
            form_data = form_data.part("response_format", fmt_bytes);
        };

        if let Some(user) = &opts.user {
            let user_bytes = multipart::Part::bytes(user.as_bytes().to_owned());
            form_data = form_data.part("user", user_bytes);
        }

        let res = self
            .client
            .post(&uri)
            .header("Authorization", format!("Bearer {api_key}"))
            .multipart(form_data)
            .send()
            .await?;

        let images = res.json().await?;

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
    pub async fn test_create_image() {
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
            .create_image(&opts)
            .await
            .expect("error creating image");
    }

    #[tokio::test]
    pub async fn test_edit_img() {
        initialize();
        let api_key = env::var("OPENAI_API_KEY").expect("error loading API key");
        let client = OpenAIClient::new(&api_key, "https://api.openai.com/v1");
        let toad_img_path = env::current_dir()
            .expect("error getting current directory")
            .into_os_string()
            .into_string()
            .expect("error converting directory path to string")
            + "/assets/toad.png";

        let img = std::fs::read(&toad_img_path).expect("error loading image");

        let opts = EditImgOptions::default(
            "toad.png",
            img,
            ImgType::Png,
            "Please change the background to dark blue",
        );

        let images = client.edit_img(&opts).await.expect("error editing image");
        println!("{:#?}", images);
    }

    #[tokio::test]
    pub async fn test_create_img_variations() {
        initialize();
        let api_key = env::var("OPENAI_API_KEY").expect("error loading API key");
        let client = OpenAIClient::new(&api_key, "https://api.openai.com/v1");
        let toad_img_path = env::current_dir()
            .expect("error getting current directory")
            .into_os_string()
            .into_string()
            .expect("error converting directory path to string")
            + "/assets/toad.png";

        let img = std::fs::read(&toad_img_path).expect("error loading image");

        let opts = CreateImgVariationsOptions::default("toad.png", img, ImgType::Png);

        let images = client
            .create_img_variations(&opts)
            .await
            .expect("error creating image variations");

        println!("{:#?}", images);
    }
}
