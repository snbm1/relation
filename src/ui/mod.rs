#![warn(clippy::never_loop)]

#[cfg(feature = "tui")]
mod tui;

use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(not(feature = "daemon"))]
use crate::datamanager::app::App;

#[cfg(feature = "daemon")]
use crate::datamanager::async_app::App;

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

        /// Set route rules [<action>:<type>:<value>]
        #[arg(long)]
        route: Option<Vec<String>>,

        /// Manage route rules [<action>:<value>:<value>]
        #[arg(long)]
        manage: Option<Vec<String>>,

        /// Set as tunnel (also name as VPN)
        #[arg(short, long)]
        tun: bool,

        /// Replace file if exist
        #[arg(short, long)]
        rewrite: bool,

        /// Set a custom name of config
        #[arg(long)]
        name: Option<String>,
    },
    /// Manage settings of config
    Manage {
        /// Config endentifier
        value: Option<ConfigEn>,

        /// Set dns servers
        #[arg(long, short)]
        dns: Option<Vec<String>>,

        /// Set route rules [<action>:<type>:<value>]
        #[arg(long, short)]
        route: Option<Vec<String>>,

        /// Manage route rules [<action>:<value>:<value>]
        #[arg(long, short)]
        manage: Option<Vec<String>>,

        /// Set a custom name of config
        #[arg(long)]
        name: Option<String>,

        #[arg(long)]
        print: bool,
    },

    /// Dispay list of possible configs
    List,

    /// Get daemon status
    #[cfg(feature = "daemon")]
    Status,

    /// Run terminal user interface
    #[cfg(feature = "tui")]
    Tui,

    /// Remove config
    Remove {
        /// Config endentifier
        value: Option<ConfigEn>,
    },

    /// Run application with selected config
    Run {
        /// Config endentifier
        value: Option<ConfigEn>,

        #[arg(long, short)]
        unable_system_proxy: bool,

        #[cfg(feature = "daemon")]
        #[arg(long, short)]
        quiet: bool,
    },

    ///Stop proxy
    #[cfg(feature = "daemon")]
    Stop,

    ///Stop and shutdown proxy
    #[cfg(feature = "daemon")]
    Quit,
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
    pub fn run(&mut self, manager: &mut App) -> Result<()> {
        match &self.command {
            Commands::Add {
                url,
                dns,
                route,
                manage,
                tun,
                rewrite,
                name,
            } => {
                if let Some(value) = url {
                    if !tun {
                        manager.handler_mut().default();
                    } else {
                        manager.handler_mut().default_tun();
                    }
                    manager.handler_mut().set_outbound_from_url(value)?;
                    if let Some(value) = dns {
                        manager.handler_mut().add_dns_servers(value)?;
                    }
                    if let Some(value) = route {
                        manager.handler_mut().add_route_rules(value)?;
                    }
                    if let Some(value) = manage {
                        manager.handler_mut().manage(value)?;
                    }
                    if *rewrite
                        && let Some(value) = name
                        && manager.exist_config(value) > 0
                    {
                        manager.remove_config(&value)?;
                    } else if *rewrite
                        && manager.exist_config(&manager.handler_ref().get_outbound_tag()?) > 0
                    {
                        manager.remove_config(&manager.handler_ref().get_outbound_tag()?)?;
                    }
                    manager.add_config(name.clone())?;
                }
            }
            Commands::Manage {
                value,
                dns,
                route,
                manage,
                name,
                print,
            } => {
                let rr = match value {
                    Some(x) => match x {
                        ConfigEn::Text(t) => manager.set_handler_config_by_name(&t),
                        ConfigEn::Number(n) => {
                            manager.set_handler_config_by_number(*n as usize - 1)
                        }
                    },
                    None => manager.set_handler_config_by_current(),
                };

                if let Err(x) = rr {
                    return Err(anyhow!(x));
                }

                if *print {
                    println!(
                        "DNS:\n{}",
                        yaml_serde::to_string(manager.handler_ref().get_dns_ref())?
                    );
                    println!(
                        "INBOUND:\n{}",
                        yaml_serde::to_string(manager.handler_ref().get_inbound_ref())?
                    );
                    println!(
                        "OUTBOUND:\n{}",
                        yaml_serde::to_string(manager.handler_ref().get_outbound_ref())?
                    );
                    println!(
                        "ROUTE:\n{}",
                        yaml_serde::to_string(manager.handler_ref().get_route_ref())?
                    );
                }

                if let Some(value) = dns {
                    manager.handler_mut().add_dns_servers(value)?;
                }
                if let Some(value) = route {
                    manager.handler_mut().add_route_rules(value)?;
                }
                if let Some(value) = manage {
                    manager.handler_mut().manage(value)?;
                }
                if let Some(value) = name {
                    manager.rename_config(value.clone())?;
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
            Commands::Remove { value } => {
                let rr = match value {
                    Some(x) => match x {
                        ConfigEn::Text(t) => manager.remove_config(t),
                        ConfigEn::Number(n) => manager.remove_config_by_number(*n as usize),
                    },
                    None => {
                        for i in manager.get_list() {
                            manager.remove_config(&i)?;
                        }
                        Ok(())
                    }
                };

                if let Err(x) = rr {
                    return Err(anyhow!(x));
                }
            }
            Commands::Run {
                value,
                unable_system_proxy,
                #[cfg(feature = "daemon")]
                quiet,
            } => {
                #[cfg(feature = "daemon")]
                if !quiet {
                    setup_signal_handler();
                }

                #[cfg(not(feature = "daemon"))]
                setup_signal_handler();

                let rr = match value {
                    Some(x) => match x {
                        ConfigEn::Text(t) => manager.run_app(Some(&t), None, *unable_system_proxy),
                        ConfigEn::Number(n) => {
                            manager.run_app(None, Some(*n as usize - 1), *unable_system_proxy)
                        }
                    },
                    None => manager.run_app(None, None, *unable_system_proxy),
                };

                if let Err(x) = rr {
                    return Err(anyhow!(x));
                }

                #[cfg(feature = "daemon")]
                if !quiet {
                    while RUNNING.load(Ordering::SeqCst) {
                        manager.read_logs();
                        for line in manager.get_new_logs() {
                            println!("{}", line);
                        }
                        std::thread::sleep(std::time::Duration::from_millis(200));
                    }

                    if let Err(x) = manager.stop_app() {
                        println!("{x}");
                    }
                }

                #[cfg(not(feature = "daemon"))]
                {
                    while RUNNING.load(Ordering::SeqCst) {
                        manager.read_logs();
                        for line in manager.get_new_logs() {
                            println!("{}", line);
                        }
                        std::thread::sleep(std::time::Duration::from_millis(200));
                    }

                    if let Err(x) = manager.stop_app() {
                        println!("{x}");
                    }
                }
            }

            #[cfg(feature = "daemon")]
            Commands::Stop => {
                if let Err(x) = manager.stop_app() {
                    println!("{x}");
                }
            }

            #[cfg(feature = "tui")]
            Commands::Tui => {
                let _ = tui::run(manager);
            }

            #[cfg(feature = "daemon")]
            Commands::Status => {
                let a = manager.get_status()?;
                if a.is_none() {
                    println!("Not started");
                } else {
                    println!("Running config: {}", a.as_ref().unwrap().file);
                    println!("System proxy: {}", a.as_ref().unwrap().sys_proxy);
                }
            }

            #[cfg(feature = "daemon")]
            Commands::Quit => manager.send_quit()?,
        }
        Ok(())
    }
}
