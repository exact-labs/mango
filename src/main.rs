mod cli;
mod helpers;

use clap::{CommandFactory, Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use colored::Colorize;
use just_macros::crashln;

#[derive(Parser)]
#[command(version, name = "mango")]
struct Cli {
    #[arg(global = true, short, long, default_value_t = helpers::get_default_kv(), help = "kv path")]
    path: String,
    #[command(subcommand)]
    command: Option<Commands>,
    #[clap(flatten)]
    verbose: Verbosity,
}

#[derive(Subcommand)]
enum Commands {
    /// Print a value from the store
    Get {
        #[command()]
        key: String,
    },
    /// Set a value from the store
    Set {
        #[command()]
        key: String,
        #[command()]
        value: String,
    },
    /// Remove a value from the store
    Del {
        #[command()]
        key: String,
    },
    /// List all values in the store
    List,
    /// Save store to csv file
    Save {
        #[command()]
        filename: String,
    },
    /// Range between values in the store
    Range {
        #[command()]
        start: String,
        #[command()]
        end: String,
    },
}

fn main() {
    helpers::create_default_kv();

    let cli = Cli::parse();
    let mut cmd = Cli::command();
    env_logger::Builder::new().filter_level(cli.verbose.log_level_filter()).init();

    match &cli.command {
        Some(Commands::Get { key }) => match cli::kv::get(&cli.path, key) {
            Ok(result) => println!("{result}"),
            Err(err) => {
                log::warn!("{err}");
                crashln!("Unable to get '{key}', does it exist?");
            }
        },

        Some(Commands::Set { key, value }) => match cli::kv::set(&cli.path, key, value) {
            Ok(()) => {
                if !cli.verbose.is_silent() {
                    println!("{}", format!("added key '{key}' with value '{value}'").green())
                }
            }
            Err(err) => {
                log::warn!("{err}");
                crashln!("{}", helpers::store_error(&cli.path, &format!("Unable to set '{key}'"), "is the store path correct?"));
            }
        },

        Some(Commands::Del { key }) => match cli::kv::remove(&cli.path, key) {
            Ok(()) => {
                if !cli.verbose.is_silent() {
                    println!("{}", format!("removed key '{}'", key).red())
                }
            }
            Err(err) => {
                log::warn!("{err}");
                crashln!("Unable to remove '{key}', does it exist?");
            }
        },

        Some(Commands::List) => match cli::kv::list(&cli.path, cli.verbose.is_silent()) {
            Ok(()) => log::info!("printing all keys from {} in silent: {} mode", &cli.path, cli.verbose.is_silent()),
            Err(err) => {
                log::warn!("{err}");
                crashln!("{}", helpers::store_error(&cli.path, "Unable to list all keys", "is the store path correct?"));
            }
        },

        Some(Commands::Range { start, end }) => match cli::kv::range(&cli.path, start, end) {
            Ok(()) => log::info!("range ({start}-{end}) keys from {} in silent: {} mode", &cli.path, cli.verbose.is_silent()),
            Err(err) => {
                log::warn!("{err}");
                crashln!("{}", helpers::store_error(&cli.path, "Unable to range between keys: ({start}-{end})", "is the store path correct?"));
            }
        },

        Some(Commands::Save { filename }) => match cli::kv::save(&cli.path, filename) {
            Ok(()) => println!("{}", format!("saved store '{}' to '{filename}'", &cli.path).green()),
            Err(err) => {
                log::warn!("{err}");
                crashln!("{}", helpers::store_error(&cli.path, "Unable to save keys to file", "is the store path correct?"));
            }
        },

        None => cmd.print_help().unwrap(),
    }
}
