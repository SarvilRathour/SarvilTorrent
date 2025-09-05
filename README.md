# Rust File Splitter & Downloader

This project is a learning exercise in **networking**, **concurrency**, and **data verification** using Rust and Tokio.  
It simulates how large files can be split into chunks, served across multiple connections, and reassembled on the client side with integrity checks.

---

##  Features
- **File splitting**: The server splits a file into multiple chunks.  
- **Multi-port serving**: Each chunk is served through a different TCP port.  
- **Concurrent download**: The client connects to all ports concurrently and downloads chunks in parallel.  
- **Reassembly**: The downloaded chunks are merged back into a single file.  
- **Verification**: Each chunk is hashed (SHA-256) on the server side and verified on the client side before writing.  

---

##  How It Works

### The Server
- Splits the file into 4 chunks.  
- Listens on ports **8000–8003**.  
- Sends a chunk along with its hash to the client.  

### The Client
- Connects to all ports concurrently using `tokio::spawn` and `join_all`.  
- Reads the metadata (hashes) before downloading.  
- Downloads all chunks and verifies them against the hash.  
- Merges verified chunks into `output_final.txt`.  

---

## 📂 Folder Structure
project-root/
├── client/
│ ├── client.rs
│ └── verification.rs
├── server/
│ ├── main.rs
│ └── verification.rs
├── Cargo.toml
└── sarvil.txt # sample file to serve

---

## 🚀 Run the Project

### 1. Clone the repository
```bash
git clone https://github.com/SarvilRathour/SarvilTorrent/
cd project-root

### Start the server
cd server
cargo run --bin server

### In another terminal, run the client
cd client
cargo run --bin client


The client will create an output_final.txt file that matches the original.





Example Output
Server:
Server:0
Server:1
Server:2
Server:3
The server is connecting to 8000
The server is connecting to 8001
The server is connecting to 8002
The server is connecting to 8003

Client:
Trying to connect to port:8000
Connection established with port:8000
"Hash matched: true"
...
File reassembled into output_final.txt
