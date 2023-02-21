use std::net::TcpListener;
use zero2prod_axum_examples::run;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind port");

    // Run the server we defined in lib
    run(listener).await.unwrap();
}
