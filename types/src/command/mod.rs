pub mod sys;

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
    pub fn to_cmd(&self) -> [u8; 2] {
        [self.subsystem as u8, self.id]
    }
}

pub trait Command {
    const ID: CommandID;
    const REQUEST_TYPE: CommandType;
    const RESPONSE_TYPE: CommandType;
}

pub mod ser {
    pub trait Command: super::Command {
        fn is_empty(&self) -> bool {
            self.len() < 1
        }
        fn len(&self) -> u8;
        fn data(&self) -> Vec<u8>;
        fn serialize(&self) -> Vec<u8> {
            let mut cmd = Self::ID.to_cmd();
            cmd[0] |= Self::REQUEST_TYPE as u8;

            let mut ret = Vec::with_capacity(self.len() as usize + 3);
            ret.push(self.len());
            ret.extend(cmd);
            ret.extend(self.data().into_iter().rev());  // See Z-stack Monitor and Test API, 2.
            ret
        }
    }
}

pub mod de {
    use crate::command::CommandType;

    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("unknown error")]
        Unknown,

        #[error("input ended unexpectedly")]
        UnexpectedEOF,
        #[error("mismatched command type, expected: {expected:?}, actual: {actual}")]
        MismatchedType {
            expected: CommandType,
            actual: u8,
        },
        #[error("mismatched command id, expected: {expected:?}, actual: {actual:?}")]
        MismatchedID {
            expected: [u8; 2],
            actual: [u8; 2],
        },
        #[error("error while parsing bytes `{0:?}`")]
        Parse(Vec<u8>),
    }

    pub trait Command: super::Command {
        type Output;
        fn to_output(&self, data_frame: Vec<u8>) -> Result<Self::Output, Error>;
        fn deserialize(&self, mut input: Vec<u8>) -> Result<Self::Output, Error> {
            if input.len() < 3 {
                return Err(Error::UnexpectedEOF);
            }
            let mut cmd = [input[1], input[2]];

            const COMMAND_TYPE_FLAG: u8 = 0b11100000u8;
            let command_type = cmd[0] & COMMAND_TYPE_FLAG;
            cmd[0] &= !COMMAND_TYPE_FLAG;
            if command_type != Self::RESPONSE_TYPE as u8 {
                return Err(Error::MismatchedType {
                    expected: Self::RESPONSE_TYPE,
                    actual: command_type,
                });
            }
            if cmd != Self::ID.to_cmd() {
                return Err(Error::MismatchedID { expected: Self::ID.to_cmd(), actual: cmd });
            }

            let data_frame = input.drain(3..)
                .rev() // See Z-stack Monitor and Test API, 2.
                .collect::<Vec<_>>();
            self.to_output(data_frame)
        }
    }
}
