use core::time;
use std::io::{Read, Write};
use std::net::TcpStream;

mod cmd;
mod pid;
mod obd;

use pid::PID;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:5054")?;
    stream.set_read_timeout(Some(time::Duration::from_secs(1)));
    stream.write_all(b"01 0C\r")?;

    let mut test = PID::new();
    test.set_command(b"010C");
    println!("PID Command: {}", test.get_command());

    // Get response

    let mut buffer = [0u8, 128];
    let mut response = String::new();
    loop {
        let bytes_read = stream.read(&mut buffer).unwrap_or(0);
        if bytes_read <= 0 {
            println!("Zero bytes read. Breaking.");
            break;
        }

        let text = String::from_utf8_lossy(&buffer[..bytes_read]);
        let trimmed = text.trim();
        if trimmed.ends_with(">") {
            break;
        }

        response.push_str(&trimmed);
    }

    let chunks = response
        .as_bytes()
        .chunks(2)
        .map(|pair| std::str::from_utf8(pair).unwrap_or(""))
        .collect::<Vec<&str>>();
    let formatted = chunks.join(" ");
    
    let chunk_a = chunks.get(4).unwrap();
    let chunk_b = chunks.get(5).unwrap();
    let a = i32::from_str_radix(&chunk_a, 16).unwrap_or(0);
    let b = i32::from_str_radix(&chunk_b, 16).unwrap_or(0);

    println!("Response: {}", formatted);
    println!("A: {}", a);
    println!("B: {}", b);
    let rpm = ((256 * a) + b) / 4;
    println!("RPM: {}", rpm);
    stream.shutdown(std::net::Shutdown::Both)?;
    Ok(())
}
