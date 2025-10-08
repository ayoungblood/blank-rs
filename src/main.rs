// src/main.rs
// blank-rs - Blank Rust CLI tool
// Author: Akira Youngblood

mod helpers;
mod add;

use color_eyre::eyre::{bail, Context, Ok, Result};
use color_eyre::config::{HookBuilder, Theme};
use clap::{Parser, Subcommand, CommandFactory};
use lazy_static::lazy_static;
use tracing::{trace, debug, info, warn, error};
use tracing_subscriber::{Registry, Layer};
use tracing_subscriber::fmt::Layer as FmtLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::filter::LevelFilter;
use std::fs::File;
use color_eyre::owo_colors::OwoColorize;
use crate::add::*;

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
    /// Color output
    #[arg(short, long, default_value_t, value_enum, global=true)]
    color: clap::ColorChoice,

    /// Log to file (optionally specify filename)
    #[arg(short, long, value_name = "FILE")]
    log: Option<Option<String>>,

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

fn main()  -> Result<()> {
    let opt: Opt = Opt::parse();

    // Configure color_eyre based on color option
    let theme = match opt.color {
        clap::ColorChoice::Never => {
            Theme::new() // no colors
        },
        clap::ColorChoice::Always | clap::ColorChoice::Auto => {
            Theme::dark()
        },
    };
    HookBuilder::new()
        .theme(theme)
        .install()?;

    // Create stderr tracing layer
    let stderr_layer = FmtLayer::new()
        .with_writer(std::io::stderr)
        .without_time()
        .with_ansi(match opt.color {
            clap::ColorChoice::Auto => true,
            clap::ColorChoice::Always => true,
            clap::ColorChoice::Never => false,
        })
        .with_filter(match opt.verbose {
            0 => LevelFilter::INFO,
            1 => LevelFilter::DEBUG,
            _ => LevelFilter::TRACE,
        });

    // Conditionally create logfile tracing layer
    let file_layer = if opt.log.is_some() {
        let log_file_name = match &opt.log {
            Some(Some(name)) => name.clone(),
            _ => format!("{}.log", env!("CARGO_PKG_NAME")),
        };
        let log_file = File::create(log_file_name)?;
        Some(FmtLayer::new()
            .with_writer(log_file)
            .with_ansi(match opt.color {
                clap::ColorChoice::Auto => false,
                clap::ColorChoice::Always => true,
                clap::ColorChoice::Never => false,
            })
            .with_filter(LevelFilter::TRACE))
    } else {
        None
    };

    // Build subscriber and set as default
    let subscriber = Registry::default()
        .with(stderr_layer)
        .with(file_layer);
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    debug!("debug");
    info!("info {}:{} {:?}", file!(),function!(), opt);
    warn!("warn");
    error!("error");

    println!("regular text");
    println!("test of {}, {}, {} ({})", "red".red().underline(), "green".green().bold(), "blue".blue().italic(), "hidden".dimmed().on_purple());

    match &opt.command {
        Some(Commands::Add { a, b }) => {
            let c = try_add(&a, &b).context("try_add failed")?;
            println!("{} + {} = {}", a, b, c);
            Ok(())
        },
        None => {
            // arg_required_else_help doesn't work with flags, print possible subcommands here
            let app = Opt::command();
            let subcommands: Vec<String> = app
                .get_subcommands()
                .map(|cmd| cmd.get_name().to_string())
                .collect();
            println!("No command specified. Possible commands: {}", subcommands.join(", "));
            Ok(())
        }
    }
}
