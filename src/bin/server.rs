use bel_poc::{db, data};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use anyhow::Result;

async fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => {
                println!("connection lost!");
                break;
            }
            Ok(n) => {
                println!("received packet! size = {}", n);
                if let Err(e) = stream.write_all(&buffer[..n]).await {
                    eprintln!("Failed to write to client: {}", e);
                    break;
                }
                println!(
                    "sent packet! msg = {}",
                    String::from_utf8_lossy(&buffer[..n])
                );
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
	// db::put(fake_entries)?;
	let edge_list = db::get(1, 99)?;
	println!("{edge_list:?}");
	db::put(fake_entries)?;
	let edge_list = db::get(1, 99)?;
	println!("{edge_list:?}");

	// let mut tmp: Vec<(u32, u32)> = Vec::new();
	// let pmap = data::group_entries(&fake_entries);
	// for (key, value) in pmap {
	// 	println!("group id: {}", key);
	// 	for &v in value.iter() {
	// 		println!("    item id: {}", v.item_id);
	// 	}
	// 	let rmap = data::build_relationship_map(&value);
	// 	println!("\n");
	// 	let edges = data::edge_list_map_from_relationship_map(rmap);
	// 	for (id, related) in edges {
	// 		// if key == 5 && id == 4 {
	// 		// 	println!("related = {related:?}");
	// 		// 	tmp = related;
	// 		// } else {
	// 		// 	println!("key = {key}, id = {id}");
	// 		// }
	// 		// let filename = format!("{id}.bel");
	// 		// let data = serde_binary::to_vec(&related, Endian::Little)?;
	// 		// db::write_bin_file(&filename, &data)?;
	// 	}
	// }

	// let bin_data = db::read_bin_file("1.bel")?;
	// let mut edge_list: Vec<(u32, u32)> = serde_binary::from_slice(&bin_data, Endian::Little)?;
	// println!("{edge_list:?}");
	// data::merge_edge_lists(&mut edge_list, &tmp);
	// println!("{edge_list:?}");

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
