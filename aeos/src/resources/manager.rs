use sysinfo::{System, SystemExt, CpuExt};
use std::sync::Arc;
use parking_lot::RwLock;

#[derive(Debug, Clone)]
pub struct ResourceUsage {
    pub cpu_percent: f64,
    pub memory_used: u64,
    pub memory_total: u64,
    pub swap_used: u64,
    pub swap_total: u64,
    pub processes_count: usize,
}

pub struct ResourceManager {
    system: Arc<RwLock<System>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            system: Arc::new(RwLock::new(System::new_all())),
        }
    }

    pub async fn init(&self) -> anyhow::Result<()> {
        tracing::info!("Initializing resource manager");
        let mut sys = self.system.write();
        sys.refresh_all();
        Ok(())
    }

    pub async fn cleanup(&self) -> anyhow::Result<()> {
        tracing::info!("Cleaning up resource manager");
        Ok(())
    }

    pub fn get_resource_usage(&self) -> ResourceUsage {
        let mut sys = self.system.write();
        sys.refresh_all();

        let cpu_percent = sys.global_cpu_info().cpu_usage() as f64;
        let memory_used = sys.used_memory();
        let memory_total = sys.total_memory();
        let swap_used = sys.used_swap();
        let swap_total = sys.total_swap();
        let processes_count = sys.processes().len();

        ResourceUsage {
            cpu_percent,
            memory_used,
            memory_total,
            swap_used,
            swap_total,
            processes_count,
        }
    }

    pub fn get_cpu_usage(&self) -> f64 {
        let sys = self.system.read();
        sys.global_cpu_info().cpu_usage() as f64
    }

    pub fn get_memory_usage(&self) -> (u64, u64) {
        let sys = self.system.read();
        (sys.used_memory(), sys.total_memory())
    }

    pub fn is_resource_available(&self, required_cpu: f64, required_memory: u64) -> bool {
        let usage = self.get_resource_usage();
        usage.cpu_percent < required_cpu && (usage.memory_total - usage.memory_used) >= required_memory
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}
