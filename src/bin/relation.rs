use anyhow::Result;
use clap::Parser;
use relation::datamanager::app::App;
use relation::ui::Cli;

fn main() -> Result<()> {
    let mut datamanager = App::new("relation")?;

    let mut cli = Cli::parse();

    cli.run(&mut datamanager)?;
    Ok(())
}
