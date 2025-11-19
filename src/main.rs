mod http;

#[tokio::main]
async fn main() {
    http::server::start_http_server().await;
}
