use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiplexConfig {
    pub enable: Option<bool>,
    pub protocol: Option<String>,
    pub max_streams: Option<u16>,
}

impl MultiplexConfig {
    pub fn new() -> MultiplexConfig {
        MultiplexConfig {
            enable: None,
            protocol: None,
            max_streams: None,
        }
    }

    pub fn check(&self) -> bool {
        match self.enable {
            None => false,
            Some(x) => match x {
                false => true,
                true => !(self.protocol.is_none() || self.max_streams.is_none()),
            },
        }
    }
}
