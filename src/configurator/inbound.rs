pub mod direct;
pub mod mixed;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Inbound {
    Direct(direct::DirectConfig),
    Mixed(mixed::MixedConfig),
}
