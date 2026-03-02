use default_net::{self, get_default_interface}; 
use std::{io}; 

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