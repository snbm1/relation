use default_net;
use std::io;


#[derive(Clone, Copy)]
pub struct Counters {
    pub rx: u64,
    pub tx: u64,
}

#[cfg(unix)]
pub fn iface_detect() -> String {
    match default_net::get_default_interface() {
        Ok(interface) => {
            return interface.name;
        }
        Err(e) => format!("Nothing"),
    }
}

#[cfg(windows)]
pub fn iface_detect() -> String {
    match default_net::get_default_interface() {
        Ok(interface) => interface
            .friendly_name
            .or(interface.description)
            .unwrap_or(interface.name),
        Err(_) => "Nothing".to_string(),
    }
}

#[cfg(unix)]
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

#[cfg(windows)]
pub fn read_iface(iface: &str) -> io::Result<Counters> {
    use std::ffi::c_void;
    use std::ptr::null_mut;

    #[repr(C)]
    struct MibIfTable2 {
        num_entries: u32,
        table: [MibIfRow2; 1],
    }

    #[repr(C)]
    struct MibIfRow2 {
        interface_luid: u64,
        interface_index: u32,
        interface_guid: [u8; 16],
        alias: [u16; 257],
        description: [u16; 257],
        physical_address_length: u32,
        physical_address: [u8; 32],
        permanent_physical_address: [u8; 32],
        mtu: u32,
        type_: u32,
        tunnel_type: u32,
        media_type: i32,
        physical_medium_type: i32,
        access_type: i32,
        direction_type: i32,
        interface_and_oper_status_flags: u8,
        oper_status: i32,
        admin_status: i32,
        media_connect_state: i32,
        network_guid: [u8; 16],
        connection_type: i32,
        transmit_link_speed: u64,
        receive_link_speed: u64,
        in_octets: u64,
        in_ucast_pkts: u64,
        in_nucast_pkts: u64,
        in_discards: u64,
        in_errors: u64,
        in_unknown_protos: u64,
        in_ucast_octets: u64,
        in_multicast_octets: u64,
        in_broadcast_octets: u64,
        out_octets: u64,
        out_ucast_pkts: u64,
        out_nucast_pkts: u64,
        out_discards: u64,
        out_errors: u64,
        out_ucast_octets: u64,
        out_multicast_octets: u64,
        out_broadcast_octets: u64,
        out_qlen: u64,
    }

    #[link(name = "iphlpapi")]
    unsafe extern "system" {
    fn GetIfTable2(table: *mut *mut MibIfTable2) -> u32;
    fn FreeMibTable(memory: *mut c_void);
}

    struct TableGuard(*mut MibIfTable2);

    impl Drop for TableGuard {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe {
                    FreeMibTable(self.0.cast());
                }
            }
        }
    }

    let mut table: *mut MibIfTable2 = null_mut();

    let result = unsafe { GetIfTable2(&mut table) };
    if result != 0 {
        return Err(io::Error::from_raw_os_error(result as i32));
    }

    let _guard = TableGuard(table);

    let count = unsafe { (*table).num_entries as usize };
    let rows = unsafe { std::slice::from_raw_parts((*table).table.as_ptr(), count) };

    for row in rows {
        let alias = wide_null_to_string(&row.alias);
        let description = wide_null_to_string(&row.description);

        if alias == iface || description == iface {
            return Ok(Counters {
                rx: row.in_octets,
                tx: row.out_octets,
            });
        }
    }

    Ok(Counters { rx: 0, tx: 0 })
}

#[cfg(windows)]
fn wide_null_to_string(buf: &[u16]) -> String {
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..len])
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
