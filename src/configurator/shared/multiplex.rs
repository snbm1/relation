use anyhow::{Context, Result, anyhow, bail};
use macros::auto_skip_none;
use serde::{Deserialize, Serialize};

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

pub fn build_multiplex(max_streams: Option<u16>) -> Result<Option<MultiplexConfig>> {
    let Some(max_streams) = max_streams else {
        return Ok(None);
    };

    let mut multiplex = MultiplexConfig::new();

    multiplex.enable = Some(true);
    multiplex.protocol = Some("h2mux".to_string());
    multiplex.max_streams = Some(max_streams);

    if !multiplex.check() {
        bail!("Invalid multiplex configuration");
    }

    Ok(Some(multiplex))
}
