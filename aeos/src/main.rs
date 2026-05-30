use std::sync::Arc;
use std::path::PathBuf;
use axum::{
    routing::{get, post, delete, put},
    Router,
};
use tower_http::cors::CorsLayer;
use tracing_subscriber;
use tokio::sync::RwLock;

mod core;
mod agents;
mod scheduler;
mod resources;
mod communication;
mod storage;
mod ui;
mod config;

use core::kernel::Kernel;
use resources::manager::ResourceManager;
use scheduler::task_scheduler::TaskScheduler;
use communication::message_bus::MessageBus;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("aeos=debug".parse()?),
        )
        .init();

    tracing::info!("Starting AI Agent Operating System (AEOS)");

    // Create core components
    let config = config::Config::load()?;
    let resource_manager = Arc::new(ResourceManager::new());
    let message_bus = Arc::new(MessageBus::new());
    let task_scheduler = Arc::new(TaskScheduler::new());
    let kernel = Arc::new(RwLock::new(Kernel::new(
        resource_manager.clone(),
        message_bus.clone(),
        task_scheduler.clone(),
    )));

    // Initialize kernel
    {
        let mut k = kernel.write().await;
        k.initialize().await?;
    }

    // Build API routes
    let app = Router::new()
        // Agent routes
        .route("/api/agents", get(ui::handlers::agents::list_agents))
        .route("/api/agents", post(ui::handlers::agents::create_agent))
        .route("/api/agents/:id", get(ui::handlers::agents::get_agent))
        .route("/api/agents/:id", delete(ui::handlers::agents::delete_agent))
        .route("/api/agents/:id/run", post(ui::handlers::agents::run_agent))
        .route("/api/agents/:id/stop", post(ui::handlers::agents::stop_agent))
        
        // Task routes
        .route("/api/tasks", get(ui::handlers::tasks::list_tasks))
        .route("/api/tasks", post(ui::handlers::tasks::create_task))
        .route("/api/tasks/:id", get(ui::handlers::tasks::get_task))
        .route("/api/tasks/:id", delete(ui::handlers::tasks::delete_task))
        
        // System routes
        .route("/api/system/status", get(ui::handlers::system::system_status))
        .route("/api/system/resources", get(ui::handlers::system::resource_status))
        .route("/api/system/logs", get(ui::handlers::system::system_logs))
        
        // Dashboard UI
        .route("/", get(ui::handlers::dashboard::dashboard))
        .route("/dashboard", get(ui::handlers::dashboard::dashboard))
        
        .layer(CorsLayer::permissive())
        .with_state(Arc::new(AppState {
            kernel: kernel.clone(),
            resource_manager,
            message_bus,
            task_scheduler,
        }));

    // Start web server
    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", config.web_port))
        .await?;
    
    tracing::info!("🚀 AEOS Web Server running on http://0.0.0.0:{}", config.web_port);

    axum::serve(listener, app).await?;

    Ok(())
}

pub struct AppState {
    pub kernel: Arc<RwLock<Kernel>>,
    pub resource_manager: Arc<ResourceManager>,
    pub message_bus: Arc<MessageBus>,
    pub task_scheduler: Arc<TaskScheduler>,
}
