use anyhow::Result;

#[cfg(feature = "autocomlete")]
use clap::CommandFactory;
#[cfg(feature = "autocomlete")]
use clap_complete::env::CompleteEnv;

use clap::Parser;
use relation::ui::Cli;

use relation::datamanager::app::App;

fn main() -> Result<()> {
    let mut datamanager = App::new("relation")?;

    #[cfg(feature = "autocomlete")]
    CompleteEnv::with_factory(Cli::command).complete();

    let mut cli = Cli::parse();

    cli.run(&mut datamanager)?;
    Ok(())
}
