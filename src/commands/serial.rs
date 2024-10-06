use serialport::{SerialPort, TTYPort};
use stblib::utilities::sleep;
use std::io::{Read, Write};

fn serial_session(device: &str, mut baud_rate: u32) -> Result<TTYPort, serialport::Error> {
    println!("Opening serial port: {}", device);

    if cfg!(target_os = "macos") {
        baud_rate = 0; // Force baud rate to 0 on macOS. This is because normally ioctl would set the baud rate
    } else if baud_rate == 0 {
        baud_rate = 9600; // Set to 9600 only if it was initially 0 on other OS
    }

    let port = TTYPort::open(&serialport::new(device, baud_rate));

    match port {
        Ok(p) => {
            println!("Serial port opened: {:?}", p);
            Ok(p)
        }
        Err(e) => {
            println!("Error opening serial port: {:?}", e);
            Err(e)
        }
    }
}

pub(crate) fn test_serial() {
    let mut serial = serial_session("/dev/ttys002", 9600).unwrap();
    serial
        .set_timeout(std::time::Duration::from_millis(10000))
        .unwrap();

    sleep(3);
    serial.write("matteo".as_bytes()).unwrap();
    serial.write("\n".as_bytes()).unwrap();
    sleep(1);
    serial.write("abc".as_bytes()).unwrap();
    serial.write("\n".as_bytes()).unwrap();
    sleep(5);

    let mut buffer = [0; 128];
    let bytes_read = serial.read(&mut buffer).unwrap();

    if let Ok(text) = std::str::from_utf8(&buffer[..bytes_read]) {
        println!("Read {} bytes:\n{}", bytes_read, text);
    } else {
        println!("Invalid UTF-8 sequence");
    }
}
