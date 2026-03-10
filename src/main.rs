use clap::Parser;

use louarch::run;
use louarch::utils::cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    run(cli)
}
