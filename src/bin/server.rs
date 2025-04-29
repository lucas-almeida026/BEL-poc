use bel_poc::{data, db, query};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use anyhow::Result;

async fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => {
                println!("connection lost!");
                break;
            }
            Ok(n) => {
                let q = String::from(String::from_utf8_lossy(&buffer[..n]));
				match query::parse_query(&q) {
					Ok(q) => {
						// println!("q: {q:?}");
						let mut result = String::from("null");
						match q {
							query::QueryKind::Get { id, top_k } => {
								match db::get(id, top_k.unwrap_or(1)) {
									Ok(r) => result = format!("{r:?}"),
									Err(_) => result = String::from("[]")
								}
							}
							query::QueryKind::Put(entries) => {
								let len = entries.len();
								match db::put(entries) {
									Ok(_) => result = format!("{}", len),
									Err(_) => result = String::from("-1")
								}
							}
						}
						let response = format!(":true,{result}");
						if let Err(e) = stream.write_all(&response.as_bytes()).await {
							eprintln!("Failed to write to client: {}", e);
							break;
						}
					}
					Err(e) => {
						let response = format!(":false,{:?}", e);
						if let Err(e) = stream.write_all(&response.as_bytes()).await {
							eprintln!("Failed to write to client: {}", e);
							break;
						}
					}
				}
            }
            Err(e) => {
                eprintln!("Failed to read from client: {}", e);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
	db::init_repository()?;

	let fake_entries = vec![
		data::Entry::new(12, 5),
		data::Entry::new(2, 5),
		data::Entry::new(11, 5),
		data::Entry::new(11, 5),
		data::Entry::new(4, 5),
		data::Entry::new(8, 5),
		data::Entry::new(12, 6),
		data::Entry::new(3, 6),
		data::Entry::new(1, 6),
		data::Entry::new(11, 6),
	];
	// println!("query: {:?}", query::parse_query("put (12:5),(2:5),(11:5),(11:5),(4:5),(8:5)"));

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
