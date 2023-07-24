use serde::{Deserialize, Serialize};
use serde_json::Value;

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
