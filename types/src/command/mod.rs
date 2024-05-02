mod reserved;
mod status;
pub mod sys;
pub mod util;

use log::debug;
pub use status::Status;

fn to_bincode_config() -> impl bincode::config::Config {
    bincode::config::standard()
        .with_little_endian()
        .with_fixed_int_encoding()
}

fn serialize_bincode<E: bincode::enc::Encode>(val: E) -> Vec<u8> {
    bincode::encode_to_vec(val, to_bincode_config()).unwrap()
}

fn deserialize_bincode<D: bincode::de::Decode>(data_frame: Vec<u8>) -> Result<D, de::Error> {
    let (ret, len): (D, usize) =
        bincode::decode_from_slice(data_frame.as_slice(), to_bincode_config())
            .map_err(|_| de::Error::UnexpectedEOF)?;
    if len != data_frame.len() {
        debug!(
            "data frame length mismatch, expected={}, actual={}",
            len,
            data_frame.len()
        );
        return Err(de::Error::UnexpectedEOF);
    }
    Ok(ret)
}

/// Type of command, 3 bits.
/// See Z-stack Monitor and Test API, 2.1.2.
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum CommandType {
    Poll = 0x00,
    /// blocking request
    SREQ = 0x20,
    /// async request
    AREQ = 0x40,
    /// blocking response
    SRSP = 0x60,
}

/// Type of command, 3 bits.
/// See Z-stack Monitor and Test API, 2.1.2.
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Subsystem {
    Reserved = 0x00,

    IFaceSYS = 0x01,
    IFaceMAC = 0x02,
    IFaceNWK = 0x03,
    IFaceAF = 0x04,
    IFaceZDO = 0x05,
    IFaceSAPI = 0x06,
    IFaceUTIL = 0x07,
    IFaceDEBUG = 0x08,
    IFaceAPP = 0x09,

    ConfigAPP = 0x0F,

    GreenPower = 0x15,
}

/// See Z-stack Monitor and Test API, 2.1.2.
pub struct CommandID {
    pub subsystem: Subsystem,
    pub id: u8,
}

impl CommandID {
    pub fn to_cmd(&self) -> [u8; 2] { [self.subsystem as u8, self.id] }
}

pub trait Command {
    const ID: CommandID;
}

pub mod ser {
    use crate::command::CommandType;

    pub trait Command: super::Command {
        const REQUEST_TYPE: CommandType;
        fn is_empty(&self) -> bool { self.len() < 1 }
        fn len(&self) -> u8;
        fn data(&self) -> Vec<u8>;
        fn serialize(&self) -> Vec<u8> {
            let mut cmd = Self::ID.to_cmd();
            cmd[0] |= Self::REQUEST_TYPE as u8;

            let mut ret = Vec::with_capacity(self.len() as usize + 3);
            ret.push(self.len());
            ret.extend(cmd);
            ret.extend(self.data());
            ret
        }
    }
}

pub mod de {
    use crate::command::reserved::{CommandNotFound, ErrorCode};
    use crate::command::CommandType;
    use log::{debug, error};

    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("unknown error")]
        Unknown,

        #[error("input ended unexpectedly")]
        UnexpectedEOF,
        #[error("mismatched command type, expected: {expected:?}, actual: {actual}")]
        MismatchedType { expected: CommandType, actual: u8 },
        #[error("mismatched command id, expected: {expected:?}, actual: {actual:?}")]
        MismatchedID { expected: [u8; 2], actual: [u8; 2] },
        #[error("error while parsing bytes `{0:?}`")]
        Parse(Vec<u8>),

        #[error("command not found")]
        CommandNotFound { error_code: ErrorCode },
    }

    pub trait Command: super::Command {
        const RESPONSE_TYPE: CommandType;
        type Output;
        fn to_output(&self, data_frame: Vec<u8>) -> Result<Self::Output, Error>;
        fn deserialize(&self, mut input: Vec<u8>) -> Result<Self::Output, Error> {
            debug!(
                "deserializing command, subsys={:?}, id={}",
                Self::ID.subsystem,
                Self::ID.id
            );
            if input.len() < 3 {
                debug!(
                    "input length too short, expected: >2, actual={}",
                    input.len()
                );
                return Err(Error::UnexpectedEOF);
            }
            let mut cmd = [input[1], input[2]];
            debug!("recv command id: {:?}", cmd);
            let data_frame = input.drain(3..).collect::<Vec<_>>();
            debug!(
                "recv response data frame: {:?}, len={}",
                data_frame,
                data_frame.len()
            );

            const COMMAND_TYPE_FLAG: u8 = 0b11100000u8;
            let command_type = cmd[0] & COMMAND_TYPE_FLAG;
            cmd[0] &= !COMMAND_TYPE_FLAG;
            if command_type == CommandNotFound::RESPONSE_TYPE as u8
                && cmd == <CommandNotFound as super::Command>::ID.to_cmd()
            {
                // TODO: handle exceptions
                debug!("command {:?} not recognized by remote", Self::ID.to_cmd());
                let error_code = CommandNotFound {}.to_output(data_frame)?;
                return Err(Error::CommandNotFound { error_code });
            }

            if command_type != Self::RESPONSE_TYPE as u8 {
                debug!(
                    "command type mismatch, expected={:?}, actual={:?}",
                    Self::RESPONSE_TYPE,
                    command_type
                );
                return Err(Error::MismatchedType {
                    expected: Self::RESPONSE_TYPE,
                    actual: command_type,
                });
            }
            if cmd != Self::ID.to_cmd() {
                debug!(
                    "command id mismatch, expected={:?}, actual={:?}",
                    Self::ID.to_cmd(),
                    cmd
                );
                return Err(Error::MismatchedID {
                    expected: Self::ID.to_cmd(),
                    actual: cmd,
                });
            }
            self.to_output(data_frame)
        }
    }
}
