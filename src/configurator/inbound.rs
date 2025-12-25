pub mod direct;
pub mod mixed;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Inbound {
    Direct(direct::DirectConfig),
}
