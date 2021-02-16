use rocket::http::{Status, Header, ContentType};
use rocket::local::asynchronous::Client;
use rocket_contrib::json::JsonValue;
use serde_json::from_str;


const TOKEN: &str = "testtoken";


#[rocket::async_test]
async fn test_get_index() {
    let client = Client::tracked(super::rocket()).await.unwrap();
    // Missing Token
    {
        let resp = client.get("/").dispatch().await;
        assert_eq!(resp.status(), Status::Unauthorized);
    }
    // Has bad token
    {
        let token = "bad-token";
        let resp = client.get("/").header(Header::new("X-TOKEN", token)).dispatch().await;
        assert_eq!(resp.status(), Status::Ok);

        let body  = resp.into_string().await.unwrap();
        let value = json!({
            "msg_code": "err_token_invalid",
            "message": "Orchestrator token is invalid!",
            "token": token,
        }).to_string();
        // @Robustness. Can this fail if values are written in different order? Doesn't seem to.
        assert_eq!(body, value);
    }
    // Has good token
    {
        let resp = client.get("/").header(Header::new("X-TOKEN", TOKEN)).dispatch().await;
        assert_eq!(resp.status(), Status::Ok);

        let body  = resp.into_string().await.unwrap();
        let value = json!({
            "msg_code": "info_root_msg",
            "message": "Hello from Orchestrator v0.0.10!",
        }).to_string();
        // @Robustness. Can this fail if values are written in different order? Doesn't seem to.
        assert_eq!(body, value);
    }
}


#[rocket::async_test]
async fn test_bad_auth_on_subscribe() {
    let client = Client::tracked(super::rocket()).await.unwrap();
    let url = "/subscription/new";
    // Missing Token
    {
        let resp = client.get(url).dispatch().await;
        assert_eq!(resp.status(), Status::Unauthorized);
    }
    // Missing Headers
    {
        let resp = client.get(url)
            .header(Header::new("X-TOKEN", TOKEN))
            .dispatch().await;
        assert_eq!(resp.status(), Status::Unauthorized);
    }
    // Missing Headers
    {
        let bad_schema = "some bad name";
        let resp = client.get(url)
            .header(Header::new("X-TOKEN", TOKEN))
            .header(Header::new("X-SERVICE", "test_service_1"))
            .header(Header::new("X-URL", "http://example.com/test_service_1"))
            .header(Header::new("X-EXPECTED-SCHEMA", bad_schema))
            .dispatch().await;
        assert_eq!(resp.status(), Status::Ok);

        let body = resp.into_string().await.unwrap();
        let value = json!({
            "msg_code": "err_schema_name_invalid",
            "message": "Could not map provided schema name to existing schema!",
            "name": &bad_schema,
        }).to_string();
        assert_eq!(body, value);
    }
}


#[rocket::async_test]
async fn test_success_on_subscribe() {
    let client = Client::tracked(super::rocket()).await.unwrap();
    let url = "/subscription/new";
    let token1: String;
    let token2: String;
    // Registering client with schema 1
    {
        let schema = "schema";
        let resp = client.get(url)
            .header(Header::new("X-TOKEN", TOKEN))
            .header(Header::new("X-SERVICE", "test_service_1"))
            .header(Header::new("X-URL", "http://example.com/test_service_1"))
            .header(Header::new("X-EXPECTED-SCHEMA", schema))
            .dispatch().await;
        assert_eq!(resp.status(), Status::Ok);

        let json: JsonValue = from_str(&resp.into_string().await.unwrap()).unwrap();
        assert_eq!(json["msg_code"], String::from("info_subscription_ok"));
        assert_eq!(json["message"], String::from("Successfully subscribed to the orchestrator!"));
        // Verify token is something
        assert!(json["new_token"].is_string());
        let token = json["new_token"].as_str().unwrap().to_string();
        assert!(!token.is_empty());
        // Save to compare later
        token1 = token;
    }
    // Registering client with schema schema 2
    {
        let schema = "schema2";
        let resp = client.get(url)
            .header(Header::new("X-TOKEN", TOKEN))
            .header(Header::new("X-SERVICE", "test_service_2"))
            .header(Header::new("X-URL", "http://example.com/test_service_2"))
            .header(Header::new("X-EXPECTED-SCHEMA", schema))
            .dispatch().await;
        assert_eq!(resp.status(), Status::Ok);

        let json: JsonValue = from_str(&resp.into_string().await.unwrap()).unwrap();
        assert_eq!(json["msg_code"], String::from("info_subscription_ok"));
        assert_eq!(json["message"], String::from("Successfully subscribed to the orchestrator!"));
        // Verify token is something
        assert!(json["new_token"].is_string());
        let token = json["new_token"].as_str().unwrap().to_string();
        assert!(!token.is_empty());
        // Save to compare later
        token2 = token;
    }
    // Finally verify token is really random
    {
        assert!(token1 != token2);
    }
}


#[rocket::async_test]
async fn test_bad_token_subscription_forward() {
    let client = Client::tracked(super::rocket()).await.unwrap();
    let url = "/subscription/forward";
    // Missing Token
    {
        let resp = client.post(url)
            .header(ContentType::JSON)
            .dispatch().await;
        assert_eq!(resp.status(), Status::Unauthorized);
    }
    // Missing Headers
    {
        let resp = client.post(url)
            .header(ContentType::JSON)
            .header(Header::new("X-TOKEN", TOKEN))
            .dispatch().await;
        assert_eq!(resp.status(), Status::Unauthorized);
    }
    // Bad token
    {
        let resp = client.post(url)
            .body("\"This is a json string!\"")
            .header(ContentType::JSON)
            .header(Header::new("X-TOKEN", "some_random_token"))
            .header(Header::new("X-SERVICE", "test_service_1"))
            .header(Header::new("X-TARGET-SERVICE", "test_service_2"))
            .dispatch().await;
        assert_eq!(resp.status(), Status::Ok);

        let body = resp.into_string().await.unwrap();
        let value = json!({
            "msg_code": "err_token_invalid",
            "message": "Service token is invalid!",
        }).to_string();
        assert_eq!(body, value);
    }
}


#[rocket::async_test]
async fn invalid_schema_subscription_forward() {
    let client = Client::tracked(super::rocket()).await.unwrap();
    let nurl = "/subscription/new";
    let furl = "/subscription/forward";
    let token: String;
    // Registering client with schema 1
    {
        let schema = "schema";
        let resp = client.get(nurl)
            .header(Header::new("X-TOKEN", TOKEN))
            .header(Header::new("X-SERVICE", "test_service_1"))
            .header(Header::new("X-URL", "http://example.com/test_service_1"))
            .header(Header::new("X-EXPECTED-SCHEMA", schema))
            .dispatch().await;
        assert_eq!(resp.status(), Status::Ok);

        let json: JsonValue = from_str(&resp.into_string().await.unwrap()).unwrap();
        assert_eq!(json["msg_code"], String::from("info_subscription_ok"));
        assert_eq!(json["message"], String::from("Successfully subscribed to the orchestrator!"));
        // Verify token is something
        assert!(json["new_token"].is_string());
        let t = json["new_token"].as_str().unwrap().to_string();
        assert!(!t.is_empty());
        // Save to compare later
        token = t;
    }
    // Testing body
    {
        let resp = client.post(furl)
            .body("\"This is a json string!\"")
            .header(ContentType::JSON)
            .header(Header::new("X-TOKEN", token.clone()))
            .header(Header::new("X-SERVICE", "test_service_1"))
            .header(Header::new("X-TARGET-SERVICE", "test_service_1"))
            .dispatch().await;
        assert_eq!(resp.status(), Status::Ok);

        let body = resp.into_string().await.unwrap();
        let value = json!({
            "msg_code": "err_schema_invalid",
            "message": "Validation error: '\"This is a json string!\"' is longer than 5 characters",
        }).to_string();
        assert_eq!(body, value);
    }
    // Testing body
    {
        let resp = client.post(furl)
            .body("{\"key\": \"value\"}")
            .header(ContentType::JSON)
            .header(Header::new("X-TOKEN", token.clone()))
            .header(Header::new("X-SERVICE", "test_service_1"))
            .header(Header::new("X-TARGET-SERVICE", "test_service_1"))
            .dispatch().await;
        assert_eq!(resp.status(), Status::Ok);

        let body = resp.into_string().await.unwrap();
        let value = json!({
            "msg_code": "err_schema_invalid",
            "message": "Validation error: '{\"key\":\"value\"}' is not of type 'string'",
        }).to_string();
        assert_eq!(body, value);
    }
}


// TODO: good auth / good forward

