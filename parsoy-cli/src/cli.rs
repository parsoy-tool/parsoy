use clap::{Parser, Subcommand, ValueEnum};

#[cfg(feature = "cli")]
use crate::{handle_error, handle_new_command};

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum Commands {
    #[command(
        about = "Creates a new project parsoy (workspace)"
    )]
    New {
        #[arg(required = true)]
        project_name: String,

        #[arg(short, long, value_enum, default_value_t = Template::Compiler, help = "Defines the project architecture")]
        template: Template,

        #[arg(long, value_enum, default_value_t = Vcs::Git, help = "The version control system must be initialized in the new project",)]
        vcs: Vcs,
    },
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
pub enum Vcs {
    Git,
    Hg,
    None,
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
pub enum Template {
    Compiler,
    Interpreter,
}

/// Use this to build a local, version-controlled `parsoy` in dependent projects.
pub async fn main() {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    match cli.command {
        Commands::New { project_name, template, vcs } => {
            handle_new_command(
                project_name,
                template,
                vcs,
                verbose,
            )
            .await
            .unwrap_or_else(handle_error)
        }
    }
}
