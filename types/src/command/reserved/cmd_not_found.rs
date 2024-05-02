use crate::command::{de, deserialize_bincode, Command, CommandID, CommandType};

use znp_macros::Command;

use super::SUBSYS;

#[derive(bincode::Decode, Debug, Clone, Copy)]
pub enum ErrorCode {
    Subsystem = 0x01,
    CommandID = 0x02,
    Parameter = 0x03,
    Length = 0x04,
}

#[derive(Command, Default, Debug, Clone)]
#[cmd(subsys = "SUBSYS", id = 0x00)]
pub struct CommandNotFound {}

impl de::Command for CommandNotFound {
    const RESPONSE_TYPE: CommandType = CommandType::SRSP;
    type Output = ErrorCode;
    fn to_output(&self, data_frame: Vec<u8>) -> Result<Self::Output, de::Error> {
        #[derive(bincode::Decode)]
        struct Rsp {
            error_code: ErrorCode,
            command_header: u16,
        }
        let rsp: Rsp = deserialize_bincode(data_frame)?;
        Ok(rsp.error_code)
    }
}
