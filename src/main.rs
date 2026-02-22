mod configurator;
mod ui;
mod datamanager;
mod bridge;

use clap::Parser;
use ui::Cli;
use configurator::Configurator;
use datamanager::App;

fn main() {
    let mut configurator = Configurator::new();

    let mut datamanager = App::new("relation");

    let mut cli = Cli::parse();

    cli.run(&mut datamanager);
}
