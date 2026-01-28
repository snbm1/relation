pub mod dialfields;
pub mod listenfields;
pub mod multiplex;
pub mod tls;
pub mod transport;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Network {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Listable<T> {
    One(T),
    Many(Vec<T>),
}

pub type ListableString = Listable<String>;
pub type ListableU16 = Listable<u16>;
