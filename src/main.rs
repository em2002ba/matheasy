use clap::Parser;
use colored::*;
use std::fs;

/// CLI calculator for quick math — Matheasy!
#[derive(Parser)]
#[command(name = "Matheasy")]
#[command(about = "A simple and fast CLI calculator", version = "0.1.0")]
struct Args {
    /// Math expression to evaluate (e.g. "5 + 2 * 3")
    expression: Option<String>,

    /// Path to a text file containing expressions (one per line)
    #[arg(short, long)]
    file: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(file) = args.file {
        match fs::read_to_string(file) {
            Ok(content) => {
                for (i, line) in content.lines().enumerate() {
                    match meval::eval_str(line) {
                        Ok(result) => println!(
                            "{} = {}",
                            line.green(),
                            result.to_string().blue()
                        ),
                        Err(e) => println!("Line {} error: {}", i + 1, e.to_string().red()),
                    }
                }
            }
            Err(e) => eprintln!("{}: {}", "File error".red(), e),
        }
    } else if let Some(expr) = args.expression {
        match meval::eval_str(&expr) {
            Ok(result) => println!(
                "{} = {}",
                expr.green(),
                result.to_string().blue()
            ),
            Err(e) => eprintln!("{}: {}", "Error".red(), e),
        }
    } else {
        println!("{}", "Please provide an expression or use --file".yellow());
    }
}


