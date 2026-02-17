#![warn(clippy::never_loop)]
use clap::{Parser, Subcommand};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::datamanager::DataManager;

use signal_hook::consts::SIGINT;
use signal_hook::iterator::Signals;

use crate::bridge;

static RUNNING: AtomicBool = AtomicBool::new(true);

fn setup_signal_handler() {
    let mut signals = Signals::new([SIGINT]).unwrap();

    std::thread::spawn(move || {
        for _ in signals.forever() {
            RUNNING.store(false, Ordering::SeqCst);
            break;
        }
    });
}

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
        #[arg(
            long,
            short,
            required_unless_present = "number",
            conflicts_with = "number"
        )]
        tag: Option<String>,

        /// Number of config
        #[arg(
            long,
            short,
            required_unless_present = "tag",
            conflicts_with = "tag",
            value_parser = clap::value_parser!(u16).range(1..)
        )]
        number: Option<u16>,
    },

    Run {
        /// Name of config
        #[arg(
            long,
            short,
            required_unless_present = "number",
            conflicts_with = "number"
        )]
        tag: Option<String>,

        /// Number of config
        #[arg(
            long,
            short,
            required_unless_present = "tag",
            conflicts_with = "tag",
            value_parser = clap::value_parser!(u16).range(1..)
        )]
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
                    println!("There are no configurations");
                } else {
                    for i in manager.get_list().iter().enumerate() {
                        println!("[{}]: {}", i.0 + 1, i.1);
                    }
                }
            }
            Commands::Remove { tag, number } => {
                if let Some(n) = tag {
                    manager.remove_config(&n);
                } else if let Some(n) = number {
                    manager.remove_config_by_number(usize::from(*n) - 1);
                }
            }
            Commands::Run { tag, number } => {
                setup_signal_handler();
                let file_path;
                if let Some(n) = tag {
                    file_path = manager.get_configs_path().join(format!("{}.json", n));
                } else if let Some(n) = number {
                    file_path = manager
                        .get_configs_path()
                        .join(format!("{}.json", manager.get_list()[*n as usize -1]))
                } else {
                    file_path = manager
                        .get_configs_path()
                        .join(format!("{}.json", manager.get_list()[0]))
                }

                bridge::enable_system_proxy_safe("127.0.0.1", 12334, false);
                bridge::start_safe(file_path.to_str().unwrap(), 255);

                while RUNNING.load(Ordering::SeqCst) {
                    std::thread::sleep(std::time::Duration::from_millis(200));
                }

                println!("Shutdown Relation");

                bridge::disable_system_proxy_safe();
                bridge::stop_safe();
            }
        }
    }
}
