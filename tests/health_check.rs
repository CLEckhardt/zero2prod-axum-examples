use zero2prod_axum_examples::run;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    // Get a random port from the OS
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    // Spawn app in the background
    // Note we are getting a server directly from `run`, not a `Result<Server, ...>`
    tokio::spawn(run(listener));

    // Return the address
    format!("http://127.0.0.1:{}", port)
}
