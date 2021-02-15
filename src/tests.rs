use rocket::local::asynchronous::Client;
use rocket::http::{Status, Header};


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
        let token = "testtoken";
        let resp = client.get("/").header(Header::new("X-TOKEN", token)).dispatch().await;
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
