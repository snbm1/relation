mod configurator;
mod ui;
use clap::Parser;
use ui::Cli;
use configurator::Configurator;

fn main() {
    let mut config = Configurator::new();

    let mut cli = Cli::parse();

    cli.run(&mut config);
}
