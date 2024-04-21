use super::{Command, ser, de, CommandID, Subsystem};
use super::{CommandType, CommandType::*};

use znp_macros::{Command, EmptyCommand};

const SUBSYS: Subsystem = Subsystem::IFaceSYS;

#[derive(Command, EmptyCommand)]
#[cmd(req_type = "SREQ", rsp_type = "SRSP", subsys = "SUBSYS", id = 0x01)]
pub struct Ping {}

impl de::Command for Ping {
    type Output = [u8; 2];

    fn to_output(&self, data_frame: Vec<u8>) -> Result<Self::Output, de::Error> {
        if data_frame.len() != 2 {
            return Err(de::Error::UnexpectedEOF);
        }
        Ok([data_frame[0], data_frame[1]])
    }
}
