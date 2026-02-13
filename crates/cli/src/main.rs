use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    fn parse() -> Self {
        Parser::parse()
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Pull a file from a remote source to a local path.
    Pull {
        /// The local path to pull the file to
        path: PathBuf,

        /// Optional URL to pull from (overrides default)
        #[arg(short, long)]
        url: Option<String>,
    },
    /// Push a local file to a remote destination.
    Push {
        /// The local path to push
        path: PathBuf,

        /// Optional URL to push to
        #[arg(short, long)]
        url: Option<String>,
    },
    /// Configure the remote URL for a specific file path.
    SetRemote {
        /// The local file path
        path: PathBuf,

        /// The remote URL to set
        url: String,
    },
}

fn main() {
    let cmd_line = Cli::parse();

    match cmd_line.command {
        Commands::Pull { path, url } => {
            println!("Pulling file at: {:?}", path);
            match url {
                Some(u) => println!("Using custom URL: {}", u),
                None => println!("Using default remote URL"),
            }
        }
        Commands::Push { path, url } => {
            println!("Pushing file from: {:?}", path);
            if let Some(u) = url {
                println!("Pushing to specific URL: {}", u);
            }
        }
        Commands::SetRemote { path, url } => {
            println!("Setting remote for {:?} to {}", path, url);
        }
    }
}
