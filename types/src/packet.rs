use crate::command;

/// See Z-stack Monitor and Test API, 2.1.1.
pub struct Packet {
    /// 0xFE, start position of packet
    start_of_frame: u8,
    /// monitor test command
    command: Vec<u8>,
    /// ensure packet integrity, XOR of `command`
    frame_check_sequence: u8,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown error")]
    Unknown,

    #[error("input ended unexpectedly")]
    UnexpectedEOF,
    #[error("mismatched packet length, expected: {expected}, actual: {actual}")]
    MismatchedLength {
        expected: usize,
        actual: usize,
    },
    #[error("frame corrupted")]
    FrameCorrupted,
}

impl Packet {
    pub fn from_command(command: impl command::ser::Command) -> Self {
        let command = command.serialize();
        let frame_check_sequence = command.iter()
            .fold(u8::MIN, |acc, &e| acc ^ e);
        Self {
            start_of_frame: 0xFE,
            command,
            frame_check_sequence,
        }
    }

    pub fn from_input(mut input: Vec<u8>) -> Result<Self, Error> {
        println!("input: {:?}", input);
        if input.len() < 5 { // input contains at least SOF (1), LEN (1), CMD (2), FCS (1)
            return Err(Error::UnexpectedEOF);
        }

        let [start_of_frame, data_len] = [input[0], input[1]];
        let packet_len = data_len as usize + 5;

        input.truncate(packet_len);
        if input.len() != packet_len {
            return Err(Error::MismatchedLength {
                expected: packet_len,
                actual: input.len(),
            });
        }
        let frame_check_sequence = input.pop().ok_or(Error::UnexpectedEOF)?;
        let command = input.drain(2..)
            .collect::<Vec<_>>();
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
