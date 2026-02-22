#![warn(clippy::never_loop)]
use clap::{Parser, Subcommand};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::datamanager::App;

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
        #[arg(long, short, conflicts_with = "number")]
        tag: Option<String>,

        /// Number of config
        #[arg(
            long,
            short,
            conflicts_with = "tag",
            value_parser = clap::value_parser!(u16).range(1..)
        )]
        number: Option<u16>,
    },

    Run {
        /// Name of config
        #[arg(long, short, conflicts_with = "number")]
        tag: Option<String>,

        /// Number of config
        #[arg(
            long,
            short,
            conflicts_with = "tag",
            value_parser = clap::value_parser!(u16).range(1..)
        )]
        number: Option<u16>,

        #[arg(long, short)]
        unable_system_proxy: bool,
    },
}

impl Cli {
    pub fn run(&mut self, manager: &mut App) {
        match &self.command {
            Commands::Add { url } => {
                if let Some(value) = url {
                    manager.handler_mut().default().set_outbound_from_url(value);
                    manager.add_config();
                }
            }
            Commands::List => {
                if manager.get_list().is_empty() {
                    println!("There are no configurations");
                } else {
                    for i in manager.get_list().iter().enumerate() {
                        println!("[{}]: {}", i.0 + 1, i.1);
                    }
                }
            }
            Commands::Remove { tag, number } => {
                if let Some(n) = tag {
                    manager.remove_config(n);
                } else if let Some(n) = number {
                    manager.remove_config_by_number(*n as usize - 1);
                } else {
                    for i in manager.get_list() {
                        manager.remove_config(&i);
                    }
                }
            }
            Commands::Run {
                tag,
                number,
                unable_system_proxy,
            } => {
                setup_signal_handler();
                manager.run_app(tag.clone(), *number, *unable_system_proxy);

                while RUNNING.load(Ordering::SeqCst) {
                    std::thread::sleep(std::time::Duration::from_millis(200));
                }

                manager.stop_app();
            }
        }
    }
}
