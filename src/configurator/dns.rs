use serde::{Deserialize, Serialize};
pub mod dnsrule;
pub mod dnsruleaction;
pub mod dnsserver;

use rellib::auto_skip_none;

use dnsserver::*;

#[auto_skip_none]
#[derive(Serialize, Deserialize, Default)]
pub struct DnsConfig {
    pub servers: Option<Vec<DnsServer>>,
    pub rules: Option<Vec<String>>,
    #[serde(rename = "final")]
    pub default: Option<String>,
    pub strategy: Option<String>,
    pub disable_cache: Option<bool>,
    pub disable_expire: Option<bool>,
    pub independent_cache: Option<bool>,
    pub cache_capacity: Option<u16>,
    pub reverse_mapping: Option<bool>,
    pub client_subnet: Option<String>,
}

impl DnsConfig {
    pub fn new() -> Self {
        DnsConfig {
            servers: Some(vec![]),
            ..Default::default()
        }
    }

    pub fn get_tag_by_type(&self, name: &str) -> Option<String> {
        self.servers
            .as_ref()
            .and_then(|s| s.iter().find(|x| x.get_type() == name))
            .map(|x| x.get_tag())
    }

    pub fn set_final_by_type(&mut self, name: &str) -> &mut Self {
        self.default = self.get_tag_by_type(name);
        self
    }

    pub fn add_server(&mut self, server: DnsServer) -> &mut Self {
        match &mut self.servers {
            Some(servers) => servers.push(server),
            None => self.servers = Some(vec![server]),
        }

        self
    }

    pub fn add_local(&mut self, tag: Option<String>) -> &mut Self {
        if let Some(name) = tag {
            self.servers
                .as_mut()
                .unwrap()
                .push(DnsServer::Local(DnsServerLocal::with_tag(name)));
        } else {
            self.servers
                .as_mut()
                .unwrap()
                .push(DnsServer::Local(DnsServerLocal::new()));
        }
        self
    }

    pub fn add_tcp(
        &mut self,
        server: Option<String>,
        server_port: Option<u16>,
        tag: Option<String>,
    ) -> &mut Self {
        if let Some(name) = tag {
            self.servers.as_mut().unwrap().push(DnsServer::Tcp(
                DnsServerTcp::with_server(server, server_port).change_tag(name),
            ));
        } else {
            self.servers
                .as_mut()
                .unwrap()
                .push(DnsServer::Tcp(DnsServerTcp::with_server(
                    server,
                    server_port,
                )));
        }
        self
    }

    pub fn add_udp(
        &mut self,
        server: Option<String>,
        server_port: Option<u16>,
        tag: Option<String>,
    ) -> &mut Self {
        if let Some(name) = tag {
            self.servers.as_mut().unwrap().push(DnsServer::Udp(
                DnsServerUdp::with_server(server, server_port).change_tag(name),
            ));
        } else {
            self.servers
                .as_mut()
                .unwrap()
                .push(DnsServer::Udp(DnsServerUdp::with_server(
                    server,
                    server_port,
                )));
        }
        self
    }

    pub fn remove_server_by_type(&mut self, name: &str) -> &mut Self {
        if let Some(servers) = self.servers.as_mut() {
            servers.retain(|x| x.get_type() != name);
        }
        self
    }

    pub fn remove_server_by_tag(&mut self, name: &str) -> &mut Self {
        if let Some(servers) = self.servers.as_mut() {
            servers.retain(|x| x.get_tag() != name);
        }
        self
    }
}
