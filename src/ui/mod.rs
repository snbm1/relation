#![warn(clippy::never_loop)]

#[cfg(feature = "tui")]
mod tui;

use clap::{Parser, Subcommand};
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::datamanager::App;

use signal_hook::consts::SIGINT;
use signal_hook::iterator::Signals;

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
    /// Add config
    Add {
        /// Config url
        #[arg(short, long)]
        url: Option<String>,

        /// Set dns servers
        #[arg(long)]
        dns: Option<Vec<String>>,

        /// Set Route rules [<type>:<value>:<action>]
        #[arg(long)]
        route: Option<Vec<String>>,

        /// Set as tunnel (also name as VPN)
        #[arg(short, long)]
        tun: bool,

        /// Set a custom name of config
        #[arg(long)]
        name: Option<String>,
    },

    /// Dispay list of possible configs
    List,

    /// Run terminal user interface
    #[cfg(feature = "tui")]
    Tui,

    /// Remove config
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

    /// Run application with selected config
    Run {
        /// Config endentifier
        value: Option<ConfigEn>,

        #[arg(long, short)]
        unable_system_proxy: bool,
    },
}

#[derive(Debug, Clone)]
enum ConfigEn {
    Number(u16),
    Text(String),
}

impl FromStr for ConfigEn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<u16>() {
            Ok(ConfigEn::Number(n))
        } else {
            Ok(ConfigEn::Text(s.to_string()))
        }
    }
}

impl Cli {
    pub fn run(&mut self, manager: &mut App) {
        match &self.command {
            Commands::Add {
                url,
                dns,
                route,
                tun,
                name,
            } => {
                if let Some(value) = url {
                    if !tun {
                        manager.handler_mut().default();
                    } else {
                        manager.handler_mut().default_tun();
                    }
                    manager.handler_mut().set_outbound_from_url(value);
                    if let Some(value) = dns {
                        manager.handler_mut().set_dns_servers(value.clone());
                    }
                    if let Some(value) = route {
                        manager.handler_mut().set_route_rules(value.clone());
                    }
                    manager.add_config(name.clone());
                }
            }
            Commands::List => {
                if manager.get_list().is_empty() {
                    println!("There are no configurations");
                } else {
                    for i in manager.get_list().iter().enumerate() {
                        println!("[{:2}]: {}", i.0 + 1, i.1);
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
                value,
                unable_system_proxy,
            } => {
                setup_signal_handler();

                match value {
                    Some(x) => match x {
                        ConfigEn::Text(t) => manager.run_app(Some(&t), None, *unable_system_proxy),
                        ConfigEn::Number(n) => {
                            manager.run_app(None, Some(*n), *unable_system_proxy)
                        }
                    },
                    None => manager.run_app(None, None, *unable_system_proxy),
                }
                .unwrap();

                while RUNNING.load(Ordering::SeqCst) {
                    manager.read_logs();
                    for line in manager.get_new_logs() {
                        println!("{}", line);
                    }
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }

                if let Err(x) = manager.stop_app() {
                    println!("{x}");
                }
            }

            #[cfg(feature = "tui")]
            Commands::Tui => {
                let _ = tui::run(manager);
            }
        }
    }
}
