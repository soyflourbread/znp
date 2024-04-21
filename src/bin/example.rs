use std::io::{Read, Write};
use std::time::Duration;
use serialport::{ClearBuffer, SerialPortType};
use znp_types::packet::Packet;

fn get_first_usb_serial() -> String {
    let ports = serialport::available_ports().unwrap();
    for port in ports {
        if let SerialPortType::UsbPort(_) = port.port_type {
            return port.port_name;
        }
    }
    String::new()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let port = get_first_usb_serial();
    let port = "/dev/tty.usbserial-110";
    println!("port: {port:?}");

    let mut tty = serialport::new(port, 9600)
        .timeout(Duration::from_millis(100))
        .open()?;
    tty.clear(ClearBuffer::All)?;

    println!("sending command...");
    let command = znp_types::command::sys::Ping {};
    let packet = Packet::from_command(command).serialize();
    tty.write_all(packet.as_slice())?;

    println!("receiving command...");
    let frame_rcv = Packet::from_reader(tty)?;
    println!("packet: {:?}", frame_rcv);

    Ok(())
}
