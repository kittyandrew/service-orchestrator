use rocket::http::{Status, Header, ContentType};
use rocket::local::asynchronous::Client;


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

