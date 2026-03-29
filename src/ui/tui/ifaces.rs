use default_net::{self, get_default_interface};
use std::fs;
use std::io;



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
        Err(e) => panic!("Error in getting interface"),
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

// pub fn ip_addr() -> String {
//     let proxy = match Proxy::new("http://127.0.0.1:12334") {
//         Ok(p) => p, 
//         Err(_) => return "proxy error".to_string(),
//     };

//     let agent = Agent::new_with_config(
//         ureq::config::Config::builder().proxy(Some((proxy))).build(),
//     );
//     let mut response = match agent.get("https://api.ipify.org").header("Connection", "close").call() {
//         Ok(r) => r,
//         Err(_) => return "responce proxy error".to_string(),
//     };

//     match response.body_mut().read_to_string() {
//         Ok(ip) => ip,
//         Err(_) => "read proxy error".to_string(),
//     }
// }

// pub fn direct_ip() -> String {
//     let mut responce = match ureq::get("https://api.ipify.org").header("Connection", "close").call() {
//         Ok(r) => r, 
//         Err(_) => return "responce error".to_string(), 
//     };

//     match responce.body_mut().read_to_string() {
//         Ok(ip) => ip, 
//         Err(_) => "read error".to_string(), 
//     }
    
// }