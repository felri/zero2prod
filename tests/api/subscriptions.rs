use crate::helpers::spawn_app;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_data() {
    // Arrange
    let app = spawn_app().await;
    let body = r#"{"email": "something@mail.com", "name": "name"}"#;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;
    // Act
    let response = app.post_subscriptions(body.to_string()).await;

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
    let json_error_vec = vec![
        r#"{"email": "something@mail.com"}"#,
        r#"{"name": "name"}"#,
        r#"{}"#,
        // r#"{"email": "not-an-email", "name": "name"}"#,
    ];

    for body in json_error_vec {
        // Act
        let response = app.post_subscriptions(body.to_string()).await;
        // Assert
        assert_eq!(400, response.status().as_u16());
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    // Arrange
    let app = spawn_app().await;
    let json_payload = vec![
        r#"{"email": "not-so-valid-email", "name": ""}"#,
        r#"{"email": "", "name": "a"}"#,
        r#"{"email": "", "name": ""}"#,
    ];

    for body in json_payload {
        // Act
        let response = app.post_subscriptions(body.to_string()).await;
        // Assert
        assert_eq!(400, response.status().as_u16());
    }
}

#[tokio::test]
async fn subscribe_sends_confirmation_email_for_valid_data() {
    // Arrange
    let app = spawn_app().await;
    let body = r#"{"email": "something@mail.com", "name": "name"}"#;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // Act
    app.post_subscriptions(body.to_string()).await;
}
