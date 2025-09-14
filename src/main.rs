// src/main.rs
// blank-rs - Blank Rust CLI tool
// Author: Akira Youngblood

mod helpers;

use anyhow::{bail, Context, Ok, Result};
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

fn try_add(a: &i32, b: &i32) -> Result<i32> {
    if *a < 0 || *b < 0 {
        bail!("Negative numbers are not supported");
    }
    Ok(*a + *b)
}

fn main()  -> Result<()> {
    let opt: Opt = Opt::parse();
    println!("{}:{} {:?}", file!(),function!(), opt);
    match &opt.command {
        Some(Commands::Add { a, b }) => {
            let c = try_add(&a, &b).context("try_add failed")?;
            println!("{} + {} = {}", a, b, c);
            Ok(())
        },
        None => {
            // we should never get here (arg_required_else_help)
            bail!("Impossible situation!")
        }
    }
}
