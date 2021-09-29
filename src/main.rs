#[macro_use]
extern crate rocket;
#[macro_use]
extern crate validator_derive;
extern crate argon2;

mod config;
mod controller;
mod handlers;
mod models;
mod services;
mod utils;

use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{Header, Status},
    response::status,
    Request, Response,
};
use serde_json::{json, Value};

#[get("/")]
fn api_home() -> status::Custom<Value> {
    let message = json!({"success": true, "message": "Authentication Server"});
    status::Custom(Status::Ok, message)
}

#[catch(404)]
fn not_found() -> status::Custom<Value> {
    let message = json!({ "success": false, "message": "Not found!" });
    status::Custom(Status::NotFound, message)
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                api_home,
                controller::sign_in,
                controller::sign_up,
                controller::find_user,
                controller::delete_user
            ],
        )
        .attach(CORS)
        .register("/", catchers![not_found])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::{ContentType, Header, Status};
    use rocket::local::asynchronous::Client;
    const REQ_BODY_SIGN_UP: &str = "first_name=kakashi&last_name=hatake&user_type=Customer&email_id=kakashi@gmail.com&password=12!@qwer";
    const REQ_BODY_LOG_IN: &str = "username=kakashi@gmail.com&password=12!@qwer";
    const REQ_BODY_DEL_USER: &str = r#"{
        "username": "kakashi@gmail.com"
    }"#;

    #[rocket::async_test]
    async fn it_works_with_correct_status_for_api_home_route() {
        let client = Client::tracked(rocket().await)
            .await
            .expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.await.status(), Status::Ok);
    }

    #[rocket::async_test]
    async fn correct_response_for_page_not_found() {
        let client = Client::tracked(rocket().await)
            .await
            .expect("valid rocket instance");
        let response = client.get("/path-does-not-exist").dispatch();
        assert_eq!(response.await.status(), Status::NotFound);
    }

    #[rocket::async_test]
    async fn it_works_for_sign_up_and_delete_user_once_signed_up() {
        let content_type =
            Header::new("Content-Type", format!("application/x-www-form-urlencoded"));
        let client = Client::tracked(rocket().await)
            .await
            .expect("valid rocket instance");
        let response = client
            .post("/auth/sign-up")
            .header(content_type.clone())
            .body(REQ_BODY_SIGN_UP.clone())
            .dispatch();
        assert_eq!(response.await.status(), Status::Ok);

        let response = client
            .post("/auth/delete-user")
            .header(ContentType::JSON)
            .body(REQ_BODY_DEL_USER.clone())
            .dispatch();
        assert_eq!(response.await.status(), Status::Ok);
    }

    #[rocket::async_test]
    async fn it_works_for_sign_in() {
        let content_type = Header::new(
            "Content-Type",
            // format!("multipart/form-data; boundary={}", BOUNDARY),
            format!("application/x-www-form-urlencoded"),
        );
        let accept = Header::new("Accept", "application/json");

        let client = Client::tracked(rocket().await)
            .await
            .expect("valid rocket instance");

        let response = client
            .post("/auth/sign-up")
            .header(content_type.clone())
            .body(REQ_BODY_SIGN_UP.clone())
            .dispatch();
        assert_eq!(response.await.status(), Status::Ok);

        let response = client
            .post("/auth/sign-in")
            .header(content_type)
            .header(accept)
            .body(REQ_BODY_LOG_IN.clone())
            .dispatch();
        assert_eq!(response.await.status(), Status::Ok);

        let response = client
            .post("/auth/delete-user")
            .header(ContentType::JSON)
            .body(REQ_BODY_DEL_USER.clone())
            .dispatch();
        assert_eq!(response.await.status(), Status::Ok);
    }
}
