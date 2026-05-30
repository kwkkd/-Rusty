use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub agent_id: String,
    pub command: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub result: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

pub struct TaskScheduler {
    tasks: Arc<DashMap<String, Task>>,
}

impl TaskScheduler {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(DashMap::new()),
        }
    }

    pub async fn init(&self) -> anyhow::Result<()> {
        tracing::info!("Initializing task scheduler");
        Ok(())
    }

    pub async fn create_task(
        &self,
        name: String,
        agent_id: String,
        command: String,
    ) -> anyhow::Result<Task> {
        let task = Task {
            id: Uuid::new_v4().to_string(),
            name,
            agent_id,
            command,
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            result: None,
        };

        self.tasks.insert(task.id.clone(), task.clone());
        tracing::info!("Task created: {}", task.id);
        Ok(task)
    }

    pub async fn get_task(&self, id: &str) -> anyhow::Result<Option<Task>> {
        Ok(self.tasks.get(id).map(|t| t.clone()))
    }

    pub async fn list_tasks(&self) -> anyhow::Result<Vec<Task>> {
        Ok(self.tasks
            .iter()
            .map(|entry| entry.value().clone())
            .collect())
    }

    pub async fn update_task_status(&self, id: &str, status: TaskStatus) -> anyhow::Result<()> {
        if let Some(mut task) = self.tasks.get_mut(id) {
            task.status = status;
            if status == TaskStatus::Running && task.started_at.is_none() {
                task.started_at = Some(Utc::now());
            }
            if matches!(status, TaskStatus::Completed | TaskStatus::Failed) {
                task.completed_at = Some(Utc::now());
            }
        }
        Ok(())
    }

    pub async fn delete_task(&self, id: &str) -> anyhow::Result<()> {
        self.tasks.remove(id);
        tracing::info!("Task deleted: {}", id);
        Ok(())
    }
}

impl Default for TaskScheduler {
    fn default() -> Self {
        Self::new()
    }
}
