mod configurator;
mod ui;
mod datamanager;

use clap::Parser;
use ui::Cli;
use configurator::Configurator;
use datamanager::DataManager;

fn main() {
    let mut configurator = Configurator::new();

    let mut datamanager = DataManager::new("relation");

    let mut cli = Cli::parse();

    cli.run(&mut datamanager);
}
