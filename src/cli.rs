use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Tyto")]
#[command(about = "State Contract Transpiler", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Build {
        #[arg(short, long, default_value = "tyto.yaml")]
        config: String,

        #[arg(short, long)]
        machine: Option<String>,
    },
}
