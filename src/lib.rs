use std::time::Duration;
use serialport::{DataBits, StopBits};
use znp_types::command::{Command, ser, de};
use znp_types::packet::{Packet, self};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown error")]
    Unknown,

    #[error("tty error: {0:?}")]
    TTY(serialport::Error),
    #[error("I/O error: {0:?}")]
    IO(std::io::Error),
    #[error("packet error: {0:?}")]
    Packet(packet::Error),
}

pub struct ZNP {
    tty: Box<dyn serialport::SerialPort>,
}

impl ZNP {
    pub fn from_port(port: String) -> Result<Self, Error> {
        let mut tty = serialport::new(port, 115200)
            .data_bits(DataBits::Eight)
            .stop_bits(StopBits::One)
            .open()
            .map_err(Error::TTY)?;
        // reset ZNP board to skip bootloader
        for (dtr, rts) in [(false, false), (false, true), (false, false)] {
            tty.write_data_terminal_ready(dtr)
                .map_err(Error::TTY)?;
            tty.write_request_to_send(rts)
                .map_err(Error::TTY)?;
        }
        std::thread::sleep(Duration::from_millis(150));

        // clear tty
        let clr = vec![0x10u8 ^ 0xFF; 256];
        tty.write_all(clr.as_slice()).map_err(Error::IO)?;
        std::thread::sleep(Duration::from_millis(2500));

        let ret = Self { tty };
        Ok(ret)
    }
}

impl ZNP {
    pub fn send_command(&mut self, command: impl ser::Command) -> Result<(), Error> {
        let packet = Packet::from_command(command)
            .serialize();
        self.tty.write_all(packet.as_slice()).map_err(Error::IO)?;

        Ok(())
    }

    pub fn recv_frame(&mut self) -> Result<Packet, Error> {
        let packet = Packet::from_reader(&mut self.tty)
            .map_err(Error::Packet)?;
        Ok(packet)
    }
}
