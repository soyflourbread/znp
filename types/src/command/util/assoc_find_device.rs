use crate::command::{de, ser, Command, CommandID, CommandType};

use znp_macros::{Command, PassRsp};

use super::SUBSYS;

#[derive(Command, PassRsp, Debug, Clone)]
#[cmd(subsys = "SUBSYS", id = 0x49)]
#[rsp(kind = "CommandType::SRSP")]
pub struct AssocFindDevice {
    /// n-th active entry in the device list
    nth_active_entry: u8,
}

impl AssocFindDevice {
    pub fn new(nth_active_entry: u8) -> Self { Self { nth_active_entry } }
}

impl ser::Command for AssocFindDevice {
    const REQUEST_TYPE: CommandType = CommandType::SREQ;
    fn len(&self) -> u8 { 1 }
    fn data(&self) -> Vec<u8> { vec![self.nth_active_entry] }
}
