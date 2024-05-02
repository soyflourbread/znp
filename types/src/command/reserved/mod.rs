mod cmd_not_found;
pub use cmd_not_found::{CommandNotFound, ErrorCode};

use crate::command::Subsystem;

const SUBSYS: Subsystem = Subsystem::Reserved;
