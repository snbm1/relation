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
