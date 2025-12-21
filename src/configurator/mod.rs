use url::Url;
mod vless;

pub trait Config {
    fn from_url(url: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    fn to_json(&self) -> Result<String, Box<dyn std::error::Error>>;
}

pub struct Configurator {
    config: Box<dyn Config>,
}

impl Configurator {
    pub fn from(input: &str) -> Result<Self, String> {
        let url = Url::parse(input).map_err(|e| e.to_string())?;

        match url.scheme() {
            "vless" => {
                let cfg = vless::VlessConfig::from_url(input).map_err(|e| e.to_string())?;
                Ok(Configurator {
                    config: Box::new(cfg),
                })
            }
            other => Err(format!("unsupported scheme: {other}")),
        }
    }
    pub fn to_json(&self) -> String {
        self.config.to_json().unwrap()
    }
}
