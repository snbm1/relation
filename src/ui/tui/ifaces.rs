use default_net::{self, get_default_interface}; 
use std::{io}; 
use std::fs; 

use crate::App; 

#[derive(Clone, Copy)]
pub struct Counters {
    pub rx: u64,
    pub tx: u64,
}

pub fn iface_detect() -> String {
    match default_net::get_default_interface() {
        Ok(interface) => {
            return interface.name;
        }
        Err(e) => panic!("Error in getting interface")
    }
}

pub fn read_iface(iface: &str) -> io::Result<Counters> {
    let text = std::fs::read_to_string("/proc/net/dev")?;

    for line in text.lines().skip(2) {
        if let Some((name, rest)) = line.split_once(':') {
            if name.trim() == iface {
                let cols: Vec<&str> = rest.split_whitespace().collect();
                let rx = cols.get(0).unwrap_or(&"0").parse().unwrap_or(0);
                let tx = cols.get(8).unwrap_or(&"0").parse().unwrap_or(0);
                return Ok(Counters { rx, tx });
            }
        }
    }

    Ok(Counters { rx: 0, tx: 0 })
}

pub fn read_logs(app: &mut App) -> Vec<String> {
    let path = app.get_data_path().join("box.log"); 

    match std::fs::read_to_string(path) {
        Ok(content) => {
            if content.trim().is_empty() {
                return vec!["No logs".to_string()];
            }
            let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
            let max = 20; 

            if lines.len() > max {
                lines = lines.split_off(lines.len() - max); 
            }
            lines
        }
        Err(_) => vec!["No logs".to_string()]
    }
}