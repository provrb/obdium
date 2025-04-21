use core::time;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread::sleep;

mod cmd;
mod obd;
mod pid;

use cmd::Command;
use pid::Response;

fn main() -> std::io::Result<()> {
    let mut obd = obd::OBD::new();
    obd.connect("127.0.0.1", "5054");
    obd.send_request(Command::new(b"010C"));
    println!("{:?}", obd.get_response().unwrap());
    sleep(time::Duration::from_secs(1));
    // // Get response

    // let mut buffer = [0u8, 128];
    // let mut response = String::new();
    // loop {
    //     let bytes_read = stream.read(&mut buffer).unwrap_or(0);
    //     if bytes_read <= 0 {
    //         println!("Zero bytes read. Breaking.");
    //         break;
    //     }

    //     let text = String::from_utf8_lossy(&buffer[..bytes_read]);
    //     let trimmed = text.trim();
    //     if trimmed.ends_with(">") {
    //         break;
    //     }

    //     response.push_str(&trimmed);
    // }

    // let chunks = response
    //     .as_bytes()
    //     .chunks(2)
    //     .map(|pair| std::str::from_utf8(pair).unwrap_or(""))
    //     .collect::<Vec<&str>>();
    // let formatted = chunks.join(" ");

    // let chunk_a = chunks.get(4).unwrap();
    // let chunk_b = chunks.get(5).unwrap();
    // let a = i32::from_str_radix(&chunk_a, 16).unwrap_or(0);
    // let b = i32::from_str_radix(&chunk_b, 16).unwrap_or(0);

    // println!("Response: {}", formatted);
    // println!("A: {}", a);
    // println!("B: {}", b);
    // let rpm = ((256 * a) + b) / 4;
    // println!("RPM: {}", rpm);
    // stream.shutdown(std::net::Shutdown::Both)?;
    Ok(())
}
