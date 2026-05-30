#[derive(Debug)]
pub struct Config {
    pub web_port: u16,
    pub web_host: String,
    pub database_url: String,
    pub log_level: String,
    pub max_agents: usize,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        Ok(Self {
            web_port: std::env::var("AEOS_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            web_host: std::env::var("AEOS_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:./aeos.db".to_string()),
            log_level: std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "info".to_string()),
            max_agents: std::env::var("AEOS_MAX_AGENTS")
                .unwrap_or_else(|_| "100".to_string())
                .parse()?,
        })
    }
}
