use futures::future::join_all;
use std::env::args;
use std::io::SeekFrom;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
mod verification;
use verification::hash_creation;
#[tokio::main]
async fn main() {
    println!("server starting");
    let mut file = File::open("../sarvil.txt").await.expect("file opening failed");
    let metadata = file.metadata().await.expect("no metadata found");
    let length = metadata.len();
    let chunks = length / 4;
    let remainder = length % 4;
    let mut handles = Vec::new();
    for i in 0..4 {
        println!("Server:{}", i);
        let mut file = File::open("../sarvil.txt").await.expect("file opening failed");
        let size = i * chunks;
        let chunk_size = if i == 3 { chunks + remainder } else { chunks };
        let current_port = 8000 + i as u16;

        let handle = tokio::spawn(async move {
            let addr = format!("127.0.0.1:{}", current_port);
            let listener = TcpListener::bind(&addr).await.unwrap();
            loop {
                let (mut stream, _) = listener.accept().await.unwrap();
                println!("The server is connecting to {}", current_port);
                let mut file = File::open("../sarvil.txt").await.expect("file opening failed");
                let _ = file.seek(SeekFrom::Start(size)).await.unwrap();
                let mut buffer = vec![0u8; chunk_size as usize];
                let _ = file.read_exact(&mut buffer).await.unwrap();
                let mut hash = hash_creation(&buffer);
                let _=stream.write_all(hash.as_bytes()).await.unwrap();
                let mut metadata=format!("{}|{}|",chunk_size,hash);
                println!("{}",hash);
                let _=stream.write_all(&mut metadata.as_bytes()).await.unwrap();
                let _ = stream.write_all(&mut buffer).await.unwrap();
            }
        });
        handles.push(handle);
    }
    join_all(handles).await;
}
