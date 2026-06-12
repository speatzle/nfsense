use clap::Parser;
use nfsense::varlink::NfsenseProxy;
use std::path::PathBuf;
use zlink::unix;

/// nfsense command-line interface
#[derive(Parser)]
#[command(name = "nfctl")]
#[command(about = "nfsense CLI", long_about = None)]
struct Cli {
    /// Path to the nfsense Varlink socket
    #[arg(short, long, default_value = "/run/nfsense/net.nfsense")]
    socket: PathBuf,

    /// The subcommand to run
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Ping the nfsense daemon
    Ping,
}

// ── Main ────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Ping) => {
            let mut conn = unix::connect(&cli.socket).await?;
            let result = conn.ping().await?;
            match result {
                Ok(pong) => println!("{}", pong.message),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
        None => {
            println!("nfctl: nfsense command-line interface");
            println!();
            println!("Usage: nfctl [OPTIONS] <COMMAND>");
            println!();
            println!("Commands:");
            println!("  ping    Ping the nfsense daemon");
        }
    }

    Ok(())
}
