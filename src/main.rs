use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let lst = TcpListener::bind("127.0.0.1:8000")
        .expect("Failed to bind port");
    let port = lst.local_addr().unwrap().port();
    println!("Listening on port {}", port);
    run(lst)?.await
}
