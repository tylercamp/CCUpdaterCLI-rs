#[macro_use]
extern crate clap;

mod cmd;

use cmd::CLIBaseOptions;


fn main() {
    let opts = CLIBaseOptions::parse();

    println!("Game base path has been set to: {}", opts.game);
}
