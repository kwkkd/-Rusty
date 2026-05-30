use crate::resources::manager::ResourceManager;
use crate::communication::message_bus::MessageBus;
use crate::scheduler::task_scheduler::TaskScheduler;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct KernelConfig {
    pub max_agents: usize,
    pub cpu_quota: f64,
    pub memory_quota: u64,
}

pub struct Kernel {
    pub id: String,
    pub resource_manager: Arc<ResourceManager>,
    pub message_bus: Arc<MessageBus>,
    pub task_scheduler: Arc<TaskScheduler>,
    pub config: KernelConfig,
    pub boot_time: chrono::DateTime<chrono::Utc>,
    pub is_running: bool,
}

impl Kernel {
    pub fn new(
        resource_manager: Arc<ResourceManager>,
        message_bus: Arc<MessageBus>,
        task_scheduler: Arc<TaskScheduler>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            resource_manager,
            message_bus,
            task_scheduler,
            config: KernelConfig {
                max_agents: 100,
                cpu_quota: 80.0,
                memory_quota: 8 * 1024 * 1024 * 1024, // 8GB
            },
            boot_time: chrono::Utc::now(),
            is_running: false,
        }
    }

    pub async fn initialize(&mut self) -> anyhow::Result<()> {
        tracing::info!("Initializing kernel: {}", self.id);
        
        // Initialize resource manager
        self.resource_manager.init().await?;
        
        // Start message bus
        self.message_bus.start().await?;
        
        // Initialize task scheduler
        self.task_scheduler.init().await?;
        
        self.is_running = true;
        tracing::info!("Kernel initialization complete");
        
        Ok(())
    }

    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        tracing::info!("Shutting down kernel");
        
        self.is_running = false;
        self.message_bus.stop().await?;
        self.resource_manager.cleanup().await?;
        
        tracing::info!("Kernel shutdown complete");
        
        Ok(())
    }

    pub fn get_uptime(&self) -> chrono::Duration {
        chrono::Utc::now() - self.boot_time
    }
}
