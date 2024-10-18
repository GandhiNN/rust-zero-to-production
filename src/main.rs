use std::net::TcpListener;

use zero_to_production::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind port");
    run(listener)?.await
}