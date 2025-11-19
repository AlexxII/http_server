use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

pub async fn start_http_server() -> io::Result<()> {
    const ADDR: &str = "127.0.0.1:8080";
    let listener = TcpListener::bind(ADDR).await?;
    println!("server start on :8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Set a connection with {}", addr);

        tokio::spawn(socket_task(socket));
    }
}

async fn socket_task(mut socket: TcpStream) {
    let mut buffer = [0u8; 1024];

    let n = match socket.read(&mut buffer).await {
        Ok(0) => return,
        Ok(n) => n,
        Err(_) => return,
    };

    let request = String::from_utf8_lossy(&buffer);
    // println!("{request}");

    let mut lines = request.lines();
    let first_line = lines.next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    let (method, path) = if parts.len() >= 2 {
        (parts[0], parts[1])
    } else {
        ("GET", "/")
    };

    println!("{} {}", method, path);
    let body = "Hello from Rust!\n".to_string();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
        body.len(),
        body
    );

    let _ = socket.write_all(response.as_bytes()).await;
}
