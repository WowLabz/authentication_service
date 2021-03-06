use chrono::NaiveDateTime;
use rocket::form::FromForm;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromFormField, Clone)]
pub enum UserType {
    Customer,
    Worker,
}

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email_id: String,
    // #[serde(skip_serializing)]
    pub password: Option<String>,
    pub user_type: UserType,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(FromForm, Serialize, Debug, Deserialize, Validate, Clone)]
pub struct RegisterUser {
    #[validate(length(min = 3))]
    pub first_name: String,
    #[validate(length(min = 3))]
    pub last_name: String,
    pub user_type: UserType,
    #[validate(email)]
    pub email_id: String,
    // #[serde(skip_serializing)]
    pub password: String,
}

#[derive(FromForm, Serialize, Debug, Deserialize, Clone)]
pub struct LoginUser {
    pub username: String,
    // #[serde(skip_serializing)]
    pub password: String,
}


#[derive(FromForm, Serialize, Debug, Deserialize, Clone)]
pub struct DeleteUser {
    pub username: String,
}

