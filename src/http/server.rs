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
    let mut lines = BufReader::new(socket).lines();
    let mut lines_pool: Vec<String> = vec![];

    while let Ok(Some(line)) = lines.next_line().await {
        lines_pool.push(line);
    }

    let first_line = lines_pool.into_iter().next().unwrap_or("".to_string());
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

    // let _ = socket.write_all(response.as_bytes()).await;
}
