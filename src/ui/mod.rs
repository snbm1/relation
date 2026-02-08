use clap::{Parser, Subcommand};

use crate::{configurator::Configurator, datamanager::DataManager};

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

    List
}

impl Cli {
    pub fn run(&mut self, manager: &mut DataManager) {
        match &self.command {
            Commands::Add { url } => {
                if let Some(value) = url {
                    manager
                        .handler_mut()
                        .default()
                        .set_outbound_from_url(value);
                    manager.add_config();
                }
            }
            Commands::List => {
                for i in manager.configs_list() {
                    println!("{}", i);
                }
            }
        }
    }
}
