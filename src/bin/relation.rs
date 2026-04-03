use anyhow::Result;
use clap::Parser;
use relation::ui::Cli;

#[cfg(not(feature = "daemon"))]
use relation::datamanager::app::App;
#[cfg(feature = "daemon")]
use relation::datamanager::async_app::App;

fn main() -> Result<()> {
    let mut datamanager = App::new("relation")?;

    let mut cli = Cli::parse();

    cli.run(&mut datamanager)?;
    Ok(())
}
