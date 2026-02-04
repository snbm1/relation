use serde::{Deserialize, Serialize};
use rellib::auto_skip_none;

#[auto_skip_none]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MultiplexConfig {
    pub enable: Option<bool>,
    pub protocol: Option<String>,
    pub max_streams: Option<u16>,
}

impl MultiplexConfig {
    pub fn new() -> MultiplexConfig {
        MultiplexConfig {
            ..Default::default()
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
