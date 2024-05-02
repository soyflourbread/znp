mod nv;
mod ping;

pub use nv::{ExNvIds, NVLength, NVRead, NvSysIds, NVID};
pub use ping::{Capability, Ping};

use crate::command::Subsystem;
const SUBSYS: Subsystem = Subsystem::IFaceSYS;
