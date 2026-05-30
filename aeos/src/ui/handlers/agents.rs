use axum::{
    extract::Path,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateAgentRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct AgentResponse {
    pub id: String,
    pub name: String,
    pub state: String,
    pub created_at: String,
}

pub async fn list_agents() -> Json<Vec<AgentResponse>> {
    Json(vec![])
}

pub async fn create_agent(
    Json(req): Json<CreateAgentRequest>,
) -> (StatusCode, Json<AgentResponse>) {
    let response = AgentResponse {
        id: uuid::Uuid::new_v4().to_string(),
        name: req.name,
        state: "Idle".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    (StatusCode::CREATED, Json(response))
}

pub async fn get_agent(Path(_id): Path<String>) -> Json<Option<AgentResponse>> {
    Json(None)
}

pub async fn delete_agent(Path(_id): Path<String>) -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn run_agent(Path(_id): Path<String>) -> StatusCode {
    StatusCode::OK
}

pub async fn stop_agent(Path(_id): Path<String>) -> StatusCode {
    StatusCode::OK
}
