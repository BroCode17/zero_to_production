//! test/health_check.rs

use std::net::TcpListener;

use reqwest;

// Launch our application in background
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Retrive the port assigned to us by OS
    let port = listener.local_addr().unwrap().port();
    let server = zero_to_production::run(listener).expect("Faild to bind address");
    let _ = tokio::spawn(server);
    println!("{port}");
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data(){
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=bro%20code&email=sign.no%40yahoo.com";
    let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request");
    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing(){
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20hello", "missing the email"),
        ("email=sign.no%40yahoo.com", "messing the name"),
        ("", "missing both name and email")
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(400, response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}


#[tokio::test]
async fn health_check_test(){
    // Arrange
    // spawn_app().await.expect("Failed to spawn our app");
    let address = spawn_app();
    // // Init
    let client = reqwest::Client::new();
    // // Act
    let response = client
          .get(&format!("{}/health_check", &address))
          .send()
          .await
          .expect("Failed to execute request");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

}




