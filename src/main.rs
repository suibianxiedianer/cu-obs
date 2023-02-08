use clap::Parser;

use cu_obs::cli::Cli;

fn main() {
    let _args = Cli::parse();
    println!("{:#?}", _args);
    println!("Hello, world!");
}
