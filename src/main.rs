mod bridge;
mod configurator;
mod datamanager;
mod ui;

use clap::Parser;
use configurator::Configurator;
use datamanager::App;
use ui::Cli;

fn main() {
    let mut datamanager = App::new("relation");

    let mut cli = Cli::parse();

    cli.run(&mut datamanager);
}
