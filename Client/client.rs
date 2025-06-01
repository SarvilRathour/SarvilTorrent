use futures::future::join_all;
use std::fmt::format;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
#[tokio::main]
async fn main() {
    let mut handles = Vec::new();

    for i in 0..4 {
        let port = 8000 + i as u16;
        let handle = tokio::spawn(async move {
            let addr = format!("127.0.0.1:{}", port);
            println!("Trying to connect to port:{}", port);
            if let Ok(mut stream) = TcpStream::connect(&addr).await {
                println!("Connection established wiht port:{}", port);
                let mut chunk = Vec::new();
                if let Ok(_) = stream.read_to_end(&mut chunk).await {
                    return Some((i, chunk));
                }
            }
            None
        });
        handles.push(handle);
    }
    let result = join_all(handles).await;

    let mut chunks = vec![Vec::new(); 4];
    for res in result {
        let (i, chunk) = res.unwrap().unwrap();
        chunks[i] = chunk
    }
    let mut output = File::create("ouput_final.txt").await.expect("failed");
    for chunk in chunks {
        output.write_all(&chunk).await.expect("failed ");
    }
}
