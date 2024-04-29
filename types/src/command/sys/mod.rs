mod nv;
mod ping;

pub use nv::{NVLength, NVRead};
pub use ping::{Capability, Ping};

use crate::command::Subsystem;
const SUBSYS: Subsystem = Subsystem::IFaceSYS;
