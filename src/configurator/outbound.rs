pub mod direct;
pub mod vless;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Outbound {
    Direct(direct::DirectConfig),
    Vless(vless::VlessConfig),
}
