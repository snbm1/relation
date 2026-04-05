use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::env::CompleteEnv;
use relation::ui::Cli;

#[cfg(not(feature = "daemon"))]
use relation::datamanager::app::App;

#[cfg(feature = "daemon")]
use relation::datamanager::async_app::App;

fn main() -> Result<()> {
    let mut datamanager = App::new("relation")?;

    CompleteEnv::with_factory(Cli::command).complete();
    let mut cli = Cli::parse();

    cli.run(&mut datamanager)?;
    Ok(())
}
