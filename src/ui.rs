use colored::Colorize;

pub fn print_header(message: &str) {
    println!();
    println!("{}  {}", "Tyto".bold().cyan(), message.dimmed());
    println!("{}", "─".repeat(50).dimmed());
}

pub fn success(message: &str) {
    println!("  {} {}", "[ok]".green().bold(), message);
}

pub fn success_lang(lang: &str, message: &str) {
    println!(
        "  {} {} {}",
        format!("[{}]", lang.to_uppercase()).cyan().bold(),
        "->".dimmed(),
        message
    );
}

pub fn warning(message: &str) {
    println!("  {} {}", "[warn]".yellow().bold(), message.yellow());
}

pub fn error(message: &str) {
    eprintln!("  {} {}", "[error]".red().bold(), message.red());
}

pub fn validation_errors(context: Option<&str>, errors: &[String]) {
    if let Some(ctx) = context {
        eprintln!(
            "\n  {} Validation failed in '{}':",
            "[error]".red().bold(),
            ctx.bold()
        );
    } else {
        eprintln!("\n  {} Validation failed:", "[error]".red().bold());
    }
    for err in errors {
        eprintln!("         {} {}", "-".dimmed(), err);
    }
    eprintln!();
}

pub fn module_header(name: &str) {
    println!();
    println!("  {} {}", "module".dimmed(), name.bold().white());
    println!("  {}", "─".repeat(40).dimmed());
}

pub fn complete(message: &str) {
    println!();
    println!("{}", "─".repeat(50).dimmed());
    println!("{}  {}", "[done]".green().bold(), message.bold());
    println!();
}

pub fn hint(message: &str) {
    println!("\n  {} {}", "hint:".blue().bold(), message.dimmed());
}

pub fn command_hint(cmd: &str) {
    println!("       {} {}", "$".dimmed(), cmd.cyan());
}

pub fn info(message: &str) {
    println!("  {} {}", "[info]".blue(), message.dimmed());
}

pub fn syntax_error(context: Option<&str>, details: &str) {
    if let Some(ctx) = context {
        eprintln!(
            "\n  {} Syntax error in '{}':",
            "[error]".red().bold(),
            ctx.bold()
        );
    } else {
        eprintln!("\n  {} Syntax error:", "[error]".red().bold());
    }
    for line in details.lines() {
        eprintln!("         {}", line);
    }
    eprintln!();
}
