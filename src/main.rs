#![allow(unused)]
mod subcmd;

use clap::{Command, FromArgMatches as _, Parser, Subcommand as _};
use subcmd::grep;

use crate::subcmd::grep::grep::grep_impl;

#[derive(Parser, Debug)]
enum SubCommands {
    Test {
        #[arg(short, long)]
        list: bool,
    },

    Grep {
        pattern: String,
        #[arg(value_name = "FILE")]
        path: std::path::PathBuf,
    },
}

fn main() {
    let cli = Command::new("pss")
        .version("0.1.0")
        .author("zhenyuan")
        .about("pss is a tool including pack, split, and sync")
        .arg_required_else_help(true);
    let cli = SubCommands::augment_subcommands(cli);

    let matches = cli.get_matches();
    let derived_subcommands = SubCommands::from_arg_matches(&matches)
        .map_err(|err| err.exit())
        .unwrap();
    println!("Derived subcommands: {:#?}", derived_subcommands);

    match derived_subcommands {
        SubCommands::Test { list } => {
            println!("Test command with list: {}", list);
        }
        SubCommands::Grep { pattern, path } => {
            grep_impl(pattern, path);
        }
    }
}
