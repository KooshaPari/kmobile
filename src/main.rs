use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::info;

use kmobile::{KMobileCli, Config};
use kmobile::device_basic::DeviceCommands;
use kmobile::simulator_basic::SimulatorCommands;
use kmobile::project::ProjectCommands;
use kmobile::testing::TestCommands;

#[derive(Parser)]
#[command(name = "kmobile")]
#[command(about = "KMobile - Comprehensive mobile development and testing automation")]
#[command(version, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, global = true)]
    config: Option<String>,

    #[arg(long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new KMobile project
    Init {
        #[arg(help = "Project name")]
        name: String,
        #[arg(long, help = "Project template")]
        template: Option<String>,
    },

    /// Device management commands
    Device {
        #[command(subcommand)]
        command: DeviceCommands,
    },

    /// Simulator management commands
    Simulator {
        #[command(subcommand)]
        command: SimulatorCommands,
    },

    /// Project management commands
    Project {
        #[command(subcommand)]
        command: ProjectCommands,
    },

    /// Testing automation commands
    Test {
        #[command(subcommand)]
        command: TestCommands,
    },

    /// Start API server
    Serve {
        #[arg(long, default_value = "3000")]
        port: u16,
        #[arg(long, default_value = "localhost")]
        host: String,
    },

    /// Start MCP server
    Mcp {
        #[arg(long, help = "MCP server configuration")]
        config: Option<String>,
    },

    /// Start TUI interface
    Tui,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize tracing
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(if args.verbose { "debug" } else { "info" })
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Load configuration
    let config = Config::load(args.config.as_deref())?;
    info!("KMobile started with config: {}", config.name());

    // Initialize CLI
    let cli = KMobileCli::new(config).await?;

    match args.command {
        Commands::Init { name, template } => {
            cli.init_project(&name, template.as_deref()).await?;
        }
        Commands::Device { command } => {
            cli.handle_device_command(command).await?;
        }
        Commands::Simulator { command } => {
            cli.handle_simulator_command(command).await?;
        }
        Commands::Project { command } => {
            cli.handle_project_command(command).await?;
        }
        Commands::Test { command } => {
            cli.handle_test_command(command).await?;
        }
        Commands::Serve { port, host } => {
            cli.start_api_server(&host, port).await?;
        }
        Commands::Mcp { config } => {
            cli.start_mcp_server(config.as_deref()).await?;
        }
        Commands::Tui => {
            cli.start_tui().await?;
        }
    }

    Ok(())
}
