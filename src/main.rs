// src/main.rs
// blank-rs - Blank Rust CLI tool
// Author: Akira Youngblood

mod helpers;

use anyhow::{bail, Context, Ok, Result};
use clap::{Parser, Subcommand};
use lazy_static::lazy_static;

lazy_static! {
    static ref VERSION: String = get_version_fancy();
}

#[derive(Debug, Parser)]
#[command(version = VERSION.as_str())]
#[command(author = "Akira Youngblood")]
#[command(about = "Rust CLI tool bootstrap project")]
#[command(help_template = "\
{before-help}{name} {version}
{about-with-newline}
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

pub fn get_version_fancy() -> String {
    if env!("GIT_AVAILABLE") == "true" {
        let base_version = env!("CARGO_PKG_VERSION");
        let git_sha = env!("GIT_SHA");
        let git_branch = env!("GIT_BRANCH");
        format!("{} ({} {})", base_version, git_sha, git_branch)
    } else {
        format!("{}", env!("CARGO_PKG_VERSION"))
    }
}

fn try_add_positive(a: &i32, b: &i32) -> Result<i32> {
    if *b < 0 {
        bail!("b < 0, unsupported ({}:{} {})", file!(), line!(), function!());
    }
    Ok(*a + *b)
}
fn try_add_negative(a: &i32, b: &i32) -> Result<i32> {
    if *b > 0 {
        bail!("b > 0, unsupported ({}:{} {})", file!(), line!(), function!());
    }
    Ok(*a + *b)
}

fn try_add(a: &i32, b: &i32) -> Result<i32> {
    let sum;
    if *a < 0 {
        sum = try_add_negative(a, b).context("try_add_negative failed")?;
    } else if *a > 0 {
        sum = try_add_positive(a, b).context("try_add_positive failed")?;
    } else {
        bail!("No supported add function for a = {} and b = {}", a, b);
    }
    Ok(sum)
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
            bail!("Impossible! You are missing a subcommand")
        }
    }
}
