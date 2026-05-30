use uuid::Uuid;
use std::collections::HashMap;
use async_trait::async_trait;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AgentState {
    Idle,
    Running,
    Paused,
    Stopped,
    Failed,
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub state: AgentState,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub task_count: usize,
    pub metadata: HashMap<String, String>,
}

impl Agent {
    pub fn new(name: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            state: AgentState::Idle,
            created_at: now,
            updated_at: now,
            cpu_usage: 0.0,
            memory_usage: 0,
            task_count: 0,
            metadata: HashMap::new(),
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self.state, AgentState::Running | AgentState::Paused)
    }
}

#[async_trait]
pub trait AgentRuntime: Send + Sync {
    async fn run(&self) -> anyhow::Result<()>;
    async fn pause(&self) -> anyhow::Result<()>;
    async fn resume(&self) -> anyhow::Result<()>;
    async fn stop(&self) -> anyhow::Result<()>;
}
