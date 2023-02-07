use clap::Parser;

use obs_tools::cli::Cli;

fn main() {
    let _args = Cli::parse();
    println!("{:#?}", _args);
    println!("Hello, world!");
}
