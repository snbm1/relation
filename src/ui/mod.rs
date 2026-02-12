use clap::{Parser, Subcommand};

use crate::{configurator::Configurator, datamanager::DataManager};

use crate::bridge;

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
        #[arg(long, required_unless_present = "number", conflicts_with = "number")]
        name: Option<String>,

        /// Number of config
        #[arg(
            long,
            required_unless_present = "name",
            conflicts_with = "name",
            value_parser = clap::value_parser!(u16).range(1..)
        )]
        number: Option<u16>,
    },

    Run {
        #[arg(long)]
        name: String,
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
                    println!("There are no configurations");
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
            Commands::Run { name } => {
                let file_path = manager.get_configs_path().join(format!("{name}.json"));
                bridge::enable_system_proxy_safe("127.0.0.1", 12334, false);
                bridge::start_safe(file_path.to_str().unwrap(), 255);
                std::thread::park();
            }
        }
    }
}
