use super::agent::{Agent, AgentState};
use dashmap::DashMap;
use std::sync::Arc;

pub struct AgentManager {
    agents: Arc<DashMap<String, Agent>>,
}

impl AgentManager {
    pub fn new() -> Self {
        Self {
            agents: Arc::new(DashMap::new()),
        }
    }

    pub async fn create_agent(&self, name: String) -> anyhow::Result<Agent> {
        let agent = Agent::new(name);
        self.agents.insert(agent.id.clone(), agent.clone());
        tracing::info!("Agent created: {}", agent.id);
        Ok(agent)
    }

    pub async fn get_agent(&self, id: &str) -> anyhow::Result<Option<Agent>> {
        Ok(self.agents.get(id).map(|a| a.clone()))
    }

    pub async fn list_agents(&self) -> anyhow::Result<Vec<Agent>> {
        Ok(self.agents
            .iter()
            .map(|entry| entry.value().clone())
            .collect())
    }

    pub async fn delete_agent(&self, id: &str) -> anyhow::Result<()> {
        self.agents.remove(id);
        tracing::info!("Agent deleted: {}", id);
        Ok(())
    }

    pub async fn update_agent_state(&self, id: &str, state: AgentState) -> anyhow::Result<()> {
        if let Some(mut agent) = self.agents.get_mut(id) {
            agent.state = state;
            agent.updated_at = chrono::Utc::now();
        }
        Ok(())
    }
}

impl Default for AgentManager {
    fn default() -> Self {
        Self::new()
    }
}
