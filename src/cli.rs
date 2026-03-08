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
    Init,

    Build {
        #[arg(short, long, default_value = "tyto.yaml")]
        config: String,

        machine: Option<String>,
    },

    Compile {
        source: String,

        #[arg(short, long)]
        langs: String,

        #[arg(short, long, default_value = ".")]
        out_dir: String,
    },
}
