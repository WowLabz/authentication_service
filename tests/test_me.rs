use authentication_server_rocket::rocket;
use ::rocket::http::{ContentType, Status};
use ::rocket::local::asynchronous::Client;

#[tokio::test]
async fn get_correct_status_response_for_api_home() {
    let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.await.status(), Status::Ok);
}
