use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer).await {
			Ok(0) => {
				println!("connection lost!");
				break;
			}
			Ok(n) => {
				println!("received message! size = {}", n);
				if let Err(e) = stream.write_all(&buffer[..n]).await {
					eprintln!("Failed to write to client: {}", e);
					break;
				}
				println!("sent message! msg = {}", String::from_utf8_lossy(&buffer[..n]));
			}
			Err(e) => {
				eprintln!("Failed to read from client: {}", e);
				break;
			}
		}
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:42069").await?;
    println!("Server listening on port 42069");

    loop {
		match listener.accept().await {
			Ok((stream, _)) => {
				println!("new connection!");
				tokio::spawn(async move {
					handle_client(stream).await;
				});
			}
			Err(e) => eprintln!("Error accepting connection: {}", e),
		}
	}
}
