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

    List,

    Remove {
        /// Name of config
        #[arg(short, long)]
        name: Option<String>,

        /// Number of config
        #[arg(long, value_parser = clap::value_parser!(u16).range(1..))]
        number: Option<u16>,
    },
}

impl Cli {
    pub fn run(&mut self, manager: &mut DataManager) {
        match &self.command {
            Commands::Add { url } => {
                if let Some(value) = url {
                    manager.handler_mut().default().set_outbound_from_url(value);
                    manager.add_config();
                }
            }
            Commands::List => {
                if manager.get_list().len() == 0 {
                    println!("No configs");
                } else {
                    for i in manager.get_list().iter().enumerate() {
                        println!("[{}]: {}", i.0 + 1, i.1);
                    }
                }
            }
            Commands::Remove { name, number } => {
                if let Some(n) = name {
                    manager.remove_config(&n);
                } else if let Some(n) = number {
                    manager.remove_config_by_number(usize::from(*n) - 1);
                }
            }
        }
    }
}
