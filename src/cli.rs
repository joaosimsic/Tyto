use clap::builder::styling::{AnsiColor, Color, Style, Styles};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tyto")]
#[command(version)]
#[command(about = "State Contract Transpiler")]
#[command(
    long_about = "Tyto is a DSL transpiler for defining state machines and generating \
                  type-safe code in multiple target languages."
)]
#[command(styles = get_styles())]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Initialize a new Tyto workspace")]
    Init,

    #[command(about = "Build all modules in the workspace")]
    Build {
        #[arg(
            short,
            long,
            default_value = "tyto.yaml",
            help = "Path to global config"
        )]
        config: String,

        #[arg(value_name = "MODULE", help = "Build only a specific module")]
        machine: Option<String>,
    },

    #[command(about = "Compile a single source file")]
    Compile {
        #[arg(value_name = "FILE", help = "Path to the source .ty file")]
        source: String,

        #[arg(
            short,
            long,
            value_name = "LANGS",
            help = "Target languages (comma-separated)"
        )]
        langs: String,

        #[arg(
            short,
            long,
            default_value = ".",
            value_name = "DIR",
            help = "Output directory"
        )]
        out_dir: String,
    },
}

fn get_styles() -> Styles {
    Styles::styled()
        .usage(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Cyan))),
        )
        .header(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Cyan))),
        )
        .literal(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green))))
        .placeholder(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan))))
        .valid(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Green))),
        )
        .invalid(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Red))),
        )
        .error(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Red))),
        )
}
