use znp_macros::Command;

use super::{de, ser, Command, CommandID, Subsystem};
use super::{CommandType, CommandType::*};

const SUBSYS: Subsystem = Subsystem::IFaceUTIL;

#[derive(Command, Debug, Clone)]
#[cmd(req_type = "SREQ", rsp_type = "SRSP", subsys = "SUBSYS", id = 0x49)]
pub struct AssocFindDevice {
    /// n-th active entry in the device list
    pub nth_active_entry: u8,
}

impl ser::Command for AssocFindDevice {
    fn len(&self) -> u8 {
        1
    }
    fn data(&self) -> Vec<u8> {
        vec![self.nth_active_entry]
    }
}

impl de::Command for AssocFindDevice {
    type Output = Vec<u8>;
    fn to_output(&self, data_frame: Vec<u8>) -> Result<Self::Output, de::Error> {
        Ok(data_frame) // passthrough
    }
}
