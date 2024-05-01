mod nv;
mod ping;

pub use nv::{NVLength, NVRead, NVID};
pub use ping::{Capability, Ping};

use crate::command::Subsystem;
const SUBSYS: Subsystem = Subsystem::IFaceSYS;
