//! test/health_check.rs

// Launch our application in background
fn spawn_app() {
    let server = zero_to_production::run().expect("Faild to bind address");
    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn dummy_test(){
    // Arrange
    // spawn_app().await.expect("Failed to spawn our app");
    spawn_app();
    // // Init
    // let client = reqwest::Client::new();
    // // Act
    // let response = client
    //       .get("http://127.0.0.1:8000/health_check")
    //       .send()
    //       .await
    //       .expect("Failed to execute request");
    // Assert
    // assert!(response.status().is_success());
    // assert_eq!(Some(0), response.content_length());

}



