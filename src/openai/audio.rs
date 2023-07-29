use serde::{Deserialize, Serialize};

use crate::OpenAIClient;

impl OpenAIClient {
    pub async fn create_transcription() {}
    pub async fn create_translation() {}
}

#[cfg(test)]
mod tests {
    use super::*;
}
