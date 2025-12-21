use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RealityConfig {
    pub enable: Option<bool>,
    pub public_key: Option<String>,
    pub short_id: Option<String>,
}

impl RealityConfig {
    pub fn new() -> Self {
        RealityConfig {
            enable: None,
            public_key: None,
            short_id: None,
        }
    }

    pub fn check(&self) -> bool {
        match self.enable {
            None => false,
            Some(x) => match x {
                false => true,
                true => !(self.public_key.is_none() || self.short_id.is_none()),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UtlsConfig {
    pub enable: Option<bool>,
    pub fingerprint: Option<String>,
}

impl UtlsConfig {
    pub fn new() -> Self {
        UtlsConfig {
            enable: None,
            fingerprint: None,
        }
    }
    pub fn check(&self) -> bool {
        match self.enable {
            None => false,
            Some(x) => match x {
                false => true,
                true => !self.fingerprint.is_none(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct TlsConfig {
    pub enable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_sni: Option<bool>,
    pub server_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utls: Option<UtlsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reality: Option<RealityConfig>,
}

impl TlsConfig {
    pub fn new() -> Self {
        TlsConfig {
            enable: None,
            disable_sni: None,
            server_name: None,
            insecure: None,
            utls: None,
            reality: None,
        }
    }

    pub fn check(&self) -> bool {
        match self.enable {
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
