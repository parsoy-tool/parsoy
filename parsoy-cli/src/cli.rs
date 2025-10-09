use clap::{Parser, Subcommand, ValueEnum};

use crate::cmd::run_new_command;

#[derive(Debug, Parser)]
#[clap(version, author)]
pub struct Cli {
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum Commands {
    #[command(
        about = "Generate a new projet",
        arg_required_else_help = true,
        display_order = 10
    )]
    New {
        project_name: String,
    }
}


#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum NewSubcommands {
    
}

pub fn main() {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    match cli.command {
        Commands::New { project_name } => {
            let _ = run_new_command(project_name);
        },
    }
}