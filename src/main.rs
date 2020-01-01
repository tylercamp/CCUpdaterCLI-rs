#[macro_use]
extern crate clap;

extern crate serde;

mod cmd;
mod features;
mod model;

use std::process;
use cmd::CLIBaseOptions;
use cmd::CLISubcommand;


fn main() {
    let opts = CLIBaseOptions::parse();

    println!("Game base path has been set to: {}", opts.game);

    let result: i32 = match opts.subcommand {
        CLISubcommand::Install(args) => features::install::run(args),
        CLISubcommand::Uninstall(args) => features::uninstall::run(args),
        CLISubcommand::Update(args) => features::update::run(args),
        CLISubcommand::List(args) => features::list::run(args),
        CLISubcommand::Outdated(args) => features::outdated::run(args)
    };

    process::exit(result);
}
