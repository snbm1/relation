use serde::{Deserialize, Serialize};
pub mod dnsrule;
pub mod dnsruleaction;
pub mod dnsserver;

use rellib::auto_skip_none;

use dnsserver::*;

#[auto_skip_none]
#[derive(Serialize, Deserialize, Default)]
pub struct DnsConfig {
    pub servers: Vec<DnsServer>,
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
            servers: vec![],
            ..Default::default()
        }
    }

    pub fn get_tag_by_type(&self, name: &str) -> Option<String> {
        self.servers
            .iter()
            .find(|x| x.get_type() == name)
            .map(|x| x.get_tag())
    }

    pub fn set_final_by_type(&mut self, name: &str) -> &mut Self {
        self.default = self.get_tag_by_type(name);
        self
    }

    pub fn add_server(&mut self, server: DnsServer) -> &mut Self {
        self.servers.push(server);
        self
    }

    pub fn add_local(&mut self, tag: Option<String>) -> &mut Self {
        if let Some(name) = tag {
            self.servers
                .push(DnsServer::Local(DnsServerLocal::with_tag(name)));
        } else {
            self.servers.push(DnsServer::Local(DnsServerLocal::new()));
        }
        self
    }

    pub fn add_tcp(
        &mut self,
        server: String,
        server_port: Option<u16>,
        tag: Option<String>,
    ) -> &mut Self {
        if let Some(name) = tag {
            self.servers.push(DnsServer::Tcp(
                DnsServerTcp::with_server(server, server_port).change_tag(name),
            ));
        } else {
            self.servers.push(DnsServer::Tcp(DnsServerTcp::with_server(
                server,
                server_port,
            )));
        }
        self
    }

    pub fn add_udp(
        &mut self,
        server: String,
        server_port: Option<u16>,
        tag: Option<String>,
    ) -> &mut Self {
        if let Some(name) = tag {
            self.servers.push(DnsServer::Udp(
                DnsServerUdp::with_server(server, server_port).change_tag(name),
            ));
        } else {
            self.servers.push(DnsServer::Udp(DnsServerUdp::with_server(
                server,
                server_port,
            )));
        }
        self
    }

    pub fn remove_server_by_type(&mut self, name: &str) -> &mut Self {
        self.servers.retain(|x| x.get_type() != name);
        self
    }

    pub fn remove_server_by_tag(&mut self, name: &str) -> &mut Self {
        self.servers.retain(|x| x.get_tag() != name);
        self
    }

    pub fn clean(&mut self) -> Vec<DnsServer> {
        let a = self.get_list();
        self.servers = vec![];
        a
    }

    pub fn get_list(&self) -> Vec<DnsServer> {
        self.servers.clone()
    }
}
