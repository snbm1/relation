use rellib::auto_skip_none;
use serde::{Deserialize, Serialize};

#[auto_skip_none]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RealityConfig {
    pub enabled: Option<bool>,
    pub public_key: Option<String>,
    pub short_id: Option<String>,
}

impl RealityConfig {
    pub fn new() -> Self {
        RealityConfig {
            ..Default::default()
        }
    }

    pub fn check(&self) -> bool {
        match self.enabled {
            None => false,
            Some(x) => match x {
                false => true,
                true => !(self.public_key.is_none() || self.short_id.is_none()),
            },
        }
    }
}

#[auto_skip_none]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UtlsConfig {
    pub enabled: Option<bool>,
    pub fingerprint: Option<String>,
}

impl UtlsConfig {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_fingerprint(addr: String) -> Self {
        Self {
            enabled: Some(true),
            fingerprint: Some(addr),
        }
    }

    pub fn check(&self) -> bool {
        match self.enabled {
            None => false,
            Some(x) => match x {
                false => true,
                true => !self.fingerprint.is_none(),
            },
        }
    }
}

#[auto_skip_none]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct TlsConfig {
    pub enabled: Option<bool>,
    pub disable_sni: Option<bool>,
    pub server_name: Option<String>,
    pub insecure: Option<bool>,
    pub utls: Option<UtlsConfig>,
    pub reality: Option<RealityConfig>,
}

impl TlsConfig {
    pub fn new() -> Self {
        TlsConfig {
            ..Default::default()
        }
    }

    pub fn with_server_name(name: String) -> Self {
        Self {
            server_name: Some(name),
            ..Default::default()
        }
    }

    pub fn add_utls(mut self, utls: UtlsConfig) -> Self {
        self.utls = Some(utls);
        self
    }

    pub fn add_reality(mut self, reality: RealityConfig) -> Self {
        self.reality = Some(reality);
        self
    }

    pub fn check(&self) -> bool {
        match self.enabled {
            None => false,
            Some(x) => match x {
                false => true,
                true => match self.server_name {
                    None => false,
                    _ => {
                        if self.utls.is_none() && self.reality.is_none() {
                            true
                        } else {
                            self.utls.as_ref().unwrap().check()
                                || self.reality.as_ref().unwrap().check()
                        }
                    }
                },
            },
        }
    }
}
