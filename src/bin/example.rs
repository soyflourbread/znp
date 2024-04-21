use std::io::{Write};
use std::time::Duration;
use serialport::{ClearBuffer, DataBits, SerialPortType, StopBits};
use znp_types::command::de::Command;
use znp_types::command::sys::Ping;
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

    let mut tty = serialport::new(port, 115200)
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One)
        .open()?;

    println!("flipping bits");
    for (dtr, rts) in [(false, false), (false, true), (false, false)] {
        tty.write_data_terminal_ready(dtr)?;
        tty.write_request_to_send(rts)?;
    }
    std::thread::sleep(Duration::from_millis(150));

    println!("resetting tty");
    let clr = vec![0x10u8 ^ 0xFF; 256];
    tty.write_all(clr.as_slice())?;
    std::thread::sleep(Duration::from_millis(2500));

    println!("pinging...");
    let command = Ping {};
    let packet = Packet::from_command(command.clone())
        .serialize();
    tty.write_all(packet.as_slice())?;

    loop {
        let mut exec_fn = || -> Result<u16, Box<dyn std::error::Error>> {
            let frame = Packet::from_reader(&mut tty)?;
            let capabilities = command.deserialize(frame.command)?;
            Ok(capabilities)
        };
        if let Ok(capabilities) = exec_fn() {
            println!("capabilities: {capabilities:#x}");
            break;
        }
        std::thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}
