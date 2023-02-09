use std::net::TcpListener;

#[tokio::test]
async fn health_check() {
    let addr = spawn_app();
    let client = reqwest::Client::new();
    let path = &format!("{}/health", &addr);

    let response = client
        .get(path)
        .send()
        .await
        .expect("failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_valid_form_data() {
    let addr = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client.post(&format!("{}/subscriptions", addr))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind listener");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("failed to run server");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
