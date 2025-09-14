// src/main.rs
// blank-rs - Blank Rust CLI tool
// Author: Akira Youngblood

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(about = "Codeplug conversion tool for radio configuration")]
#[command(help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
")]

struct Opt {
    /// Verbose mode (-v, -vv, -vvv, ...)
    #[arg(short, long, action = clap::ArgAction::Count, global=true)]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
#[command(arg_required_else_help = true)]
enum Commands {
    /// Add two integers
    Add {
        /// Integer A
        a: i32,
        /// Integer B
        b: i32,
    },
}

fn main() {
    let opt: Opt = Opt::parse();
    println!("{:?}", opt);
    match &opt.command {
        Some(Commands::Add { a, b }) => {
            println!("{} + {} = {}", a, b, a + b);
        },
        None => {
            // we should never get here (arg_required_else_help)
            println!("No command specified");
        }
    }
}
