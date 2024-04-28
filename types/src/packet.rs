use crate::command;

/// See Z-stack Monitor and Test API, 2.1.1.
#[derive(Debug, Clone)]
pub struct Packet {
    /// 0xFE, start position of packet
    pub start_of_frame: u8,
    /// monitor test command
    pub command: Vec<u8>,
    /// ensure packet integrity, XOR of `command`
    pub frame_check_sequence: u8,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown error")]
    Unknown,

    #[error("input ended unexpectedly")]
    UnexpectedEOF,
    #[error("frame corrupted")]
    FrameCorrupted,
}

impl Packet {
    pub fn from_command(command: impl command::ser::Command) -> Self {
        let command = command.serialize();
        let frame_check_sequence = command.iter().fold(u8::MIN, |acc, &e| acc ^ e);
        Self {
            start_of_frame: 0xFE,
            command,
            frame_check_sequence,
        }
    }

    pub fn from_reader(mut reader: impl std::io::Read) -> Result<Self, Error> {
        let mut start_of_frame = u8::MIN;
        reader
            .read_exact(std::slice::from_mut(&mut start_of_frame))
            .map_err(|_| Error::UnexpectedEOF)?;
        let mut data_len = u8::MIN;
        reader
            .read_exact(std::slice::from_mut(&mut data_len))
            .map_err(|_| Error::UnexpectedEOF)?;
        let mut data_frame = vec![u8::MIN; data_len as usize + 2];
        reader
            .read_exact(data_frame.as_mut_slice())
            .map_err(|_| Error::UnexpectedEOF)?;
        let mut frame_check_sequence = u8::MIN;
        reader
            .read_exact(std::slice::from_mut(&mut frame_check_sequence))
            .map_err(|_| Error::UnexpectedEOF)?;
        println!(
            "frame: {:x?}, data_len={}, sof={}, fcs={}",
            data_frame, data_len, start_of_frame, frame_check_sequence
        );

        let mut command = vec![data_len];
        command.extend(data_frame);
        let frame_check_sequence_target = command.iter().fold(u8::MIN, |acc, &e| acc ^ e);
        if frame_check_sequence != frame_check_sequence_target {
            return Err(Error::FrameCorrupted);
        }

        let ret = Self {
            start_of_frame,
            command,
            frame_check_sequence,
        };
        Ok(ret)
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut ret = vec![];
        ret.push(self.start_of_frame);
        ret.extend(self.command.clone());
        ret.push(self.frame_check_sequence);
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::packet::Packet;

    #[test]
    fn ping_request() {
        let command = crate::command::sys::Ping {};
        let expected = vec![0xFE, 0x00, 0x21, 0x01, 0x20];
        assert_eq!(Packet::from_command(command).serialize(), expected)
    }
}
