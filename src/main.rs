use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The subcommand, such as add/status, etc.
    #[clap(subcommand)]
    subcommand: CliSub,

    /// Name of the target Project/home.
    project: String,

    /// Name of the Package.
    package: String,
}

#[derive(Parser, Debug)]
struct AddArgs {
    /// A SRC rpm file path
    path: String,
}

#[derive(Parser, Debug)]
struct BinariesArgs {
    /// The Repo option.
    repo: String,

    /// The target arch, x86_64/aarch64, etc.
    arch: String,

    /// Which path to store the binaries.
    #[arg(short, long)]
    path: String,
}

#[derive(Subcommand, Debug)]
enum CliSub {
    /// Add a package from src.rpm.
    Add(AddArgs),

    /// Show the current status.
    Status,

    /// Download binaries packages if build success.
    Binaries(BinariesArgs),
}

fn main() {
    let _args = Cli::parse();
    println!("{:#?}", _args);
    println!("Hello, world!");
}
