use chrono::NaiveDateTime;
use mongodb::bson;
use rocket::form::FromForm;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use strum_macros::{EnumString, EnumVariantNames};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromFormField, Clone)]
pub enum UserType {
    Customer,
    Worker,
}

#[derive(Debug, Serialize, Deserialize, FromFormField, Clone, EnumString, EnumVariantNames)]
// #[strum(serialize_all = "kebab_case")]
pub enum UserTags {
    WebDevelopment,
    MobileDevelopment,
    MachineLearning,
    DeepLearning,
    FullStackDevelopment,
    CoreBlockchainDevelopment,
}

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub user_id: Option<bson::oid::ObjectId>,
    pub first_name: String,
    pub last_name: String,
    pub email_id: String,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub user_type: UserType,
    pub user_tags: Vec<UserTags>,
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
    pub user_tags: Vec<UserTags>,
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
