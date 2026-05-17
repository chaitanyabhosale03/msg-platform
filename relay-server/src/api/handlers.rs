use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

#[derive(Deserialize)]
pub struct UploadKeysRequest {
    pub public_key: String,
    pub signed_prekey: String,
    pub prekey_signature: String,
}

pub async fn upload_keys(Json(_payload): Json<UploadKeysRequest>) -> Json<serde_json::Value> {
    // TODO: Store prekeys in database
    // TODO: Return prekey id
    Json(serde_json::json!({ "status": "ok" }))
}

#[derive(Deserialize)]
pub struct QueueMessageRequest {
    pub recipient: String,
    pub encrypted_message: String,
}

pub async fn queue_message(Json(_payload): Json<QueueMessageRequest>) -> Json<serde_json::Value> {
    // TODO: Queue message for offline delivery
    Json(serde_json::json!({ "status": "queued" }))
}
