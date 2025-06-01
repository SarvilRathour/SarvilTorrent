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
                let file_path = format!("file{}.bin", i);
                let mut file = File::create(&file_path).await.expect("failure");
                let mut chunk = Vec::new();
                let _ = stream.read_to_end(&mut chunk).await;
                let _ = file.write_all(&chunk).await;
            }
        });
        handles.push(handle);
    }
    join_all(handles).await;
}
