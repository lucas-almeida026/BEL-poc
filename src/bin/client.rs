use std::net::TcpStream;
use std::io::{self, Write, Read};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:42069")?;
    println!("Connected to server!");

    loop {
		let mut input = String::new();
	    io::stdin().read_line(&mut input)?;
		if input.trim() == ":q" {
			println!("Exiting...");
			break;
		}

	    stream.write_all(input.as_bytes())?;
	    println!("Sent: {}", input.trim_end());

	    let mut buffer = [0; 512];
	    let n = stream.read(&mut buffer)?;
	    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
	}

    Ok(())
}
