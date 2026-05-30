use axum::{
    extract::Path,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub name: String,
    pub agent_id: String,
    pub command: String,
}

#[derive(Serialize)]
pub struct TaskResponse {
    pub id: String,
    pub name: String,
    pub status: String,
    pub created_at: String,
}

pub async fn list_tasks() -> Json<Vec<TaskResponse>> {
    Json(vec![])
}

pub async fn create_task(
    Json(req): Json<CreateTaskRequest>,
) -> (StatusCode, Json<TaskResponse>) {
    let response = TaskResponse {
        id: uuid::Uuid::new_v4().to_string(),
        name: req.name,
        status: "Pending".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    (StatusCode::CREATED, Json(response))
}

pub async fn get_task(Path(_id): Path<String>) -> Json<Option<TaskResponse>> {
    Json(None)
}

pub async fn delete_task(Path(_id): Path<String>) -> StatusCode {
    StatusCode::NO_CONTENT
}
