use clap::{Parser, Subcommand, ValueEnum};

use crate::{handle_error, new::run_new_command};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum Commands {
    #[command(about = "Create a new project.")]
    New {
        #[arg(required = true, help = "The name of a new project.")]
        project_name: String,

        #[arg(short, long, value_enum, default_value_t = ProjectTemplate::Compiler)]
        template: ProjectTemplate,
    },
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
pub enum ProjectTemplate {
    Compiler,
    Interpreter,
}

pub async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New {
            project_name,
            template,
        } => run_new_command(project_name, template)
            .await
            .unwrap_or_else(handle_error),
    }
}
