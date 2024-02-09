use midir::{MidiOutput, MidiOutputPort};
use std::thread::sleep;
use std::time::Duration;

const BAUD_RATE: u32 = 115200;
const SERIAL_DEVICE_NAME: &str = "usbmodem";

fn main() {
    // Get an output port (read from console if multiple are available)
    let midi_out = MidiOutput::new("My Output").unwrap();
    let out_port: MidiOutputPort = {
        let ports = midi_out.ports();
        match ports.len() {
            0 => panic!("No MIDI output port found!"),
            1 => {
                println!(
                    "Redirecting messages to device ({})",
                    midi_out.port_name(&ports[0]).unwrap()
                );
                ports[0].clone()
            }
            _ => {
                println!("\nAvailable output ports:");
                for (i, p) in ports.iter().enumerate() {
                    println!("{}: {}", i, midi_out.port_name(p).unwrap());
                }
                print!("Please select output port: ");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let port_index: usize = input.trim().parse().unwrap();
                ports[port_index].clone()
            }
        }
    };
    let mut midi_connection_out = midi_out.connect(&out_port, "midir-connection").unwrap();

    let serial_device_name = get_serial_device_name();
    let mut serial_port = serialport::new(serial_device_name, BAUD_RATE).open().unwrap();

    loop {
        match serial_port.bytes_to_read() {
            Ok(bytes) => {
                if bytes > 0 {
                    let mut buf: Vec<u8> = vec![0; bytes as usize];
                    serial_port.read(&mut buf).unwrap();
                    midi_connection_out.send(&buf).unwrap();
                    println!("Sent: {:?}", buf);
                }
            }
            Err(_) => {
                let name = get_serial_device_name();
                println!("Reconnected to {name}.");
                serial_port = serialport::new(name, BAUD_RATE).open().unwrap();
            }
        };
        sleep(Duration::from_millis(1));
    }
}

fn get_serial_device_name() -> String {
    loop {
        let ports = serialport::available_ports().unwrap();
        for p in ports {
            if p.port_name.contains(SERIAL_DEVICE_NAME) {
                return p.port_name;
            }
        }
        eprintln!("Serial device not available! Reconnecting... (press Ctrl+C to exit)");
        sleep(Duration::from_secs(1));
    }
}
