use clap::Parser;
use nfsense::varlink::NfsenseProxy;
use std::path::PathBuf;
use zlink::unix;

#[derive(Parser)]
#[command(name = "nfctl")]
#[command(about = "nfsense CLI", long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[arg(short, long, default_value = "/run/nfsense/net.nfsense")]
    socket: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Ping,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Ping => {
            let mut conn = unix::connect(&cli.socket).await?;
            let result = conn.ping().await?;
            match result {
                Ok(pong) => println!("{}", pong.message),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
    }

    Ok(())
}
