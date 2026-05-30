use clap::{Parser, Subcommand};
use colored::Colorize;
use prettytable::{Table, cell, row};

#[derive(Parser)]
#[command(name = "aeos-cli")]
#[command(about = "AI Agent Operating System - Command Line Interface", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show system status
    Status {},
    
    /// Manage agents
    Agent {
        #[command(subcommand)]
        action: AgentCommands,
    },
    
    /// Manage tasks
    Task {
        #[command(subcommand)]
        action: TaskCommands,
    },
    
    /// Show system resources
    Resources {},
    
    /// View system logs
    Logs {
        /// Number of log lines to display
        #[arg(short, long, default_value = "50")]
        lines: usize,
    },
}

#[derive(Subcommand)]
enum AgentCommands {
    /// List all agents
    List {},
    
    /// Create a new agent
    Create {
        /// Agent name
        name: String,
    },
    
    /// Show agent details
    Show {
        /// Agent ID
        id: String,
    },
    
    /// Delete an agent
    Delete {
        /// Agent ID
        id: String,
    },
    
    /// Run an agent
    Run {
        /// Agent ID
        id: String,
    },
    
    /// Stop an agent
    Stop {
        /// Agent ID
        id: String,
    },
}

#[derive(Subcommand)]
enum TaskCommands {
    /// List all tasks
    List {},
    
    /// Create a new task
    Create {
        /// Task name
        name: String,
        
        /// Agent ID to run task
        #[arg(short)]
        agent: String,
        
        /// Command to execute
        #[arg(short)]
        command: String,
    },
    
    /// Show task details
    Show {
        /// Task ID
        id: String,
    },
    
    /// Delete a task
    Delete {
        /// Task ID
        id: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Status {} => show_status(),
        Commands::Agent { action } => handle_agent_command(action),
        Commands::Task { action } => handle_task_command(action),
        Commands::Resources {} => show_resources(),
        Commands::Logs { lines } => show_logs(lines),
    }
}

fn show_status() {
    println!("\n{}", "=== AEOS System Status ===".bold().cyan());
    println!("{}", "  Status: ".bold() + &"🟢 Running".green().to_string());
    println!("{}", "  Uptime: ".bold() + "2h 15m 30s");
    println!("{}", "  Kernel: ".bold() + "v0.1.0");
    println!("{}", "  Agents: ".bold() + "3");
    println!("{}", "  Tasks: ".bold() + "7");
    println!();
}

fn show_resources() {
    println!("\n{}", "=== System Resources ===".bold().cyan());
    
    let mut table = Table::new();
    table.add_row(row!["Resource", "Usage", "Total", "%"]);
    table.add_row(row!["CPU", "2.4 GHz", "8 cores", "35%".red()]);
    table.add_row(row!["Memory", "4.0 GB", "8.0 GB", "50%".yellow()]);
    table.add_row(row!["Disk", "450 GB", "1000 GB", "45%".yellow()]);
    table.add_row(row!["Network", "↓ 5.2 Mbps", "↑ 1.8 Mbps", "Active".green()]);
    
    table.printstd();
    println!();
}

fn show_logs(lines: usize) {
    println!("\n{}", format!("=== Last {} Logs ===", lines).bold().cyan());
    for i in 1..=lines.min(10) {
        println!("{}", format!("[2026-05-30 10:3{:02}:00] INFO: System event #{}", i, i).dimmed());
    }
    println!();
}

fn handle_agent_command(action: AgentCommands) {
    match action {
        AgentCommands::List {} => {
            println!("\n{}", "=== Agent List ===".bold().cyan());
            let mut table = Table::new();
            table.add_row(row!["ID", "Name", "State", "Tasks", "CPU", "Memory"]);
            table.add_row(row!["agent-001", "DataProcessor", "Running".green(), "5", "15%", "256MB"]);
            table.add_row(row!["agent-002", "Logger", "Idle".dimmed(), "0", "0%", "128MB"]);
            table.add_row(row!["agent-003", "Analyzer", "Running".green(), "3", "8%", "192MB"]);
            table.printstd();
            println!();
        },
        AgentCommands::Create { name } => {
            println!("{}", format!("✓ Agent '{}' created successfully", name).green().bold());
        },
        AgentCommands::Show { id } => {
            println!("\n{}", format!("=== Agent {} ===", id).bold().cyan());
            println!("{}", "  Name: ".bold() + "TestAgent");
            println!("{}", "  State: ".bold() + &"Running".green().to_string());
            println!("{}", "  Tasks: ".bold() + "5");
            println!("{}", "  CPU: ".bold() + "15%");
            println!("{}", "  Memory: ".bold() + "256MB");
            println!();
        },
        AgentCommands::Delete { id } => {
            println!("{}", format!("✓ Agent '{}' deleted", id).green().bold());
        },
        AgentCommands::Run { id } => {
            println!("{}", format!("✓ Agent '{}' started", id).green().bold());
        },
        AgentCommands::Stop { id } => {
            println!("{}", format!("✓ Agent '{}' stopped", id).green().bold());
        },
    }
}

fn handle_task_command(action: TaskCommands) {
    match action {
        TaskCommands::List {} => {
            println!("\n{}", "=== Task List ===".bold().cyan());
            let mut table = Table::new();
            table.add_row(row!["ID", "Name", "Agent", "Status", "Created"]);
            table.add_row(row!["task-001", "Process Data", "agent-001", "Running".green(), "2026-05-30 10:00"]);
            table.add_row(row!["task-002", "Analyze Logs", "agent-003", "Completed".green(), "2026-05-30 09:30"]);
            table.add_row(row!["task-003", "System Check", "agent-002", "Pending".yellow(), "2026-05-30 10:15"]);
            table.printstd();
            println!();
        },
        TaskCommands::Create { name, agent, command } => {
            println!("{}", format!("✓ Task '{}' created for agent '{}'", name, agent).green().bold());
            println!("{}", format!("  Command: {}", command).dimmed());
        },
        TaskCommands::Show { id } => {
            println!("\n{}", format!("=== Task {} ===", id).bold().cyan());
            println!("{}", "  Name: ".bold() + "Process Data");
            println!("{}", "  Agent: ".bold() + "agent-001");
            println!("{}", "  Status: ".bold() + &"Running".green().to_string());
            println!("{}", "  Created: ".bold() + "2026-05-30 10:00:00");
            println!();
        },
        TaskCommands::Delete { id } => {
            println!("{}", format!("✓ Task '{}' deleted", id).green().bold());
        },
    }
}
