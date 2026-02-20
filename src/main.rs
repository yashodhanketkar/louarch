use clap::Parser;

use louarchrs::run;
use louarchrs::utils::cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    run(cli)
}
