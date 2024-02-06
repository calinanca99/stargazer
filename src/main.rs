use clap::Parser;
use console::style;
use stargazer::{run, Cli};

fn main() {
    if let Err(error) = run(Cli::parse()) {
        println!("{} {}", style("error:").bold().red(), error);
        std::process::exit(1);
    };
}
