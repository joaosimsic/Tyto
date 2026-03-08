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
    Init,

    Build {
        #[arg(short, long, default_value = "tyto.yaml")]
        config: String,

        #[arg(value_name = "MODULE")]
        machine: Option<String>,
    },

    Compile {
        #[arg(value_name = "FILE")]
        source: String,

        #[arg(short, long, value_name = "LANGS")]
        langs: String,

        #[arg(short, long, default_value = ".", value_name = "DIR")]
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
