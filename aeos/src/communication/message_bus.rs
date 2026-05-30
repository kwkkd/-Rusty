use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Message {
    pub id: String,
    pub from_agent: String,
    pub to_agent: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

pub struct MessageBus {
    messages: Arc<DashMap<String, Message>>,
    is_running: Arc<parking_lot::RwLock<bool>>,
}

impl MessageBus {
    pub fn new() -> Self {
        Self {
            messages: Arc::new(DashMap::new()),
            is_running: Arc::new(parking_lot::RwLock::new(false)),
        }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        let mut running = self.is_running.write();
        *running = true;
        tracing::info!("Message bus started");
        Ok(())
    }

    pub async fn stop(&self) -> anyhow::Result<()> {
        let mut running = self.is_running.write();
        *running = false;
        self.messages.clear();
        tracing::info!("Message bus stopped");
        Ok(())
    }

    pub async fn send_message(
        &self,
        from_agent: String,
        to_agent: String,
        content: String,
    ) -> anyhow::Result<Message> {
        if !*self.is_running.read() {
            return Err(anyhow::anyhow!("Message bus is not running"));
        }

        let message = Message {
            id: Uuid::new_v4().to_string(),
            from_agent,
            to_agent,
            content,
            timestamp: Utc::now(),
        };

        self.messages.insert(message.id.clone(), message.clone());
        tracing::debug!("Message sent: {}", message.id);
        Ok(message)
    }

    pub async fn get_messages(&self, agent_id: &str) -> anyhow::Result<Vec<Message>> {
        Ok(self.messages
            .iter()
            .filter(|entry| entry.value().to_agent == agent_id)
            .map(|entry| entry.value().clone())
            .collect())
    }

    pub async fn get_message(&self, id: &str) -> anyhow::Result<Option<Message>> {
        Ok(self.messages.get(id).map(|m| m.clone()))
    }

    pub async fn clear_agent_messages(&self, agent_id: &str) -> anyhow::Result<()> {
        self.messages.retain(|_, msg| msg.to_agent != agent_id);
        Ok(())
    }
}

impl Default for MessageBus {
    fn default() -> Self {
        Self::new()
    }
}
