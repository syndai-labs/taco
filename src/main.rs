//! User-facing taco binary. Milestone 0 parses the frozen grammar and stops
//! before capture, models, or the network.

#![forbid(unsafe_code)]

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use taco::error::ErrorCode;
use taco::freeze::{self, COMMANDS};
use taco::selector::{self, SOURCE_CATEGORIES};
use taco::session::{self, RunRequest, Session};

#[derive(Parser)]
#[command(
    name = "taco",
    version,
    about = "Local bilingual subtitles. This build freezes contracts and does not start capture."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Privacy and license notices. Does not touch the network.
    Notices,
    Sources {
        #[command(subcommand)]
        command: SourcesCmd,
    },
    Models {
        #[command(subcommand)]
        command: ModelsCmd,
    },
    Run {
        /// One of default, input:<uid>, system, app:<bundle-id>, pid:<pid>.
        #[arg(long = "source", action = clap::ArgAction::Append, required = true)]
        source: Vec<String>,
        #[arg(long, required = true)]
        from: String,
        #[arg(long, required = true)]
        to: String,
        #[arg(long, default_value = "cli")]
        ui: String,
        #[arg(long)]
        model_dir: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum SourcesCmd {
    List,
}

#[derive(Subcommand)]
enum ModelsCmd {
    List,
    Fetch {
        #[arg(long)]
        model_dir: Option<PathBuf>,
    },
    Verify {
        #[arg(long)]
        model_dir: Option<PathBuf>,
    },
}

fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error) => {
            let code = match error.kind() {
                clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion => 0,
                _ => 2,
            };
            error.print().ok();
            return ExitCode::from(code);
        }
    };
    match dispatch(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(code) => ExitCode::from(code as u8),
    }
}

fn dispatch(cli: Cli) -> Result<(), i32> {
    match cli.command {
        Commands::Notices => {
            let _ = taco::usage_plist().len();
            println!("taco shows subtitles in this terminal and does not upload them.");
            println!("Model digests are TBD. Terminal scrollback can keep what is displayed.");
            println!("usage keys linked: {}", taco::usage_keys_present());
            let _ = COMMANDS;
            Ok(())
        }
        Commands::Sources {
            command: SourcesCmd::List,
        } => {
            println!(
                "selectors: default | input:<device-uid> | system | app:<bundle-id> | pid:<pid>"
            );
            let names: Vec<_> = SOURCE_CATEGORIES
                .into_iter()
                .map(selector::category_name)
                .collect();
            println!("categories: {}", names.join(", "));
            if !session::platform_supported() {
                eprintln!("status enumeration_unavailable");
            }
            Ok(())
        }
        Commands::Models {
            command: ModelsCmd::List,
        } => {
            for pin in freeze::PINS {
                println!(
                    "{} {} sha256={} file={}",
                    pin.role,
                    pin.family,
                    pin.sha256.unwrap_or("TBD"),
                    pin.filename.unwrap_or("TBD")
                );
            }
            Ok(())
        }
        Commands::Models {
            command: ModelsCmd::Fetch { model_dir: _ },
        } => refuse(freeze::fetch_models()),
        Commands::Models {
            command: ModelsCmd::Verify { model_dir: _ },
        } => refuse(Err(ErrorCode::ModelsUnpinned)),
        Commands::Run {
            source,
            from,
            to,
            ui,
            model_dir: _,
        } => {
            let mut session = Session::new();
            let req = RunRequest {
                sources: &source,
                from: &from,
                to: &to,
                ui: &ui,
            };
            match session::preflight(&mut session, &req) {
                Ok(ok) => refuse(session::start_capture(&mut session, &ok)),
                Err(code) => refuse(Err(code)),
            }
        }
    }
}

fn refuse(result: Result<(), ErrorCode>) -> Result<(), i32> {
    match result {
        Ok(()) => Ok(()),
        Err(code) => {
            eprintln!("error {}", code.as_str());
            Err(code.exit_class())
        }
    }
}
