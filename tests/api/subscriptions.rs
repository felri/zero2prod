use crate::helpers::spawn_app;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let json_payload = r#"{"email": "something@mail.com", "name": "name"}"#;

    // Act
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/json")
        .body(json_payload)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "something@mail.com");
    assert_eq!(saved.name, "name");
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let json_error_vec = vec![
        r#"{"email": "something@mail.com"}"#,
        r#"{"name": "name"}"#,
        r#"{}"#,
        // r#"{"email": "not-an-email", "name": "name"}"#,
    ];

    for json_payload in json_error_vec {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/json")
            .body(json_payload)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(400, response.status().as_u16());
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let json_payload = vec![
        r#"{"email": "not-so-valid-email", "name": ""}"#,
        r#"{"email": "", "name": "a"}"#,
        r#"{"email": "", "name": ""}"#,
    ];

    for json_payload in json_payload {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/json")
            .body(json_payload)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(400, response.status().as_u16());
    }
}
