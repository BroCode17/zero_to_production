use std::net::TcpListener;

use zero_to_production::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on the server
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Field to bind");
    //  run(TcpListener::bind(listener))?.await
    Ok(())
}
