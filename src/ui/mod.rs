use clap::{Parser, Subcommand};

use crate::configurator::Configurator;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        /// Config url
        #[arg(short, long)]
        url: Option<String>,
    },
}

impl Cli {
    pub fn run(&mut self, handler: &mut Configurator) {
        match &self.command {
            Commands::Add { url } => {
                if let Some(value) = url {
                    handler
                        .default()
                        .set_outbound_from_url(value)
                        .save_to_file()
                        .unwrap();
                }
            }
        }
    }
}
