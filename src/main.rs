mod bridge;
mod configurator;
mod datamanager;
mod ui;

use anyhow::Result;
use clap::Parser;
use datamanager::App;
use ui::Cli;

fn main() -> Result<()> {
    let mut datamanager = App::new("relation")?;

    let mut cli = Cli::parse();

    cli.run(&mut datamanager)?;
    Ok(())
}
