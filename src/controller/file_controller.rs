use rocket::data::Data;
use rocket::form::FromForm;
use rocket::fs::TempFile;
use rocket::http::{ContentType, Status};
use rocket::response::status;

use rocket_download_response::DownloadResponse;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

use async_std::io::prelude::*;
use serde_json::{json, Value};
use std::{io::ErrorKind, time};

use crate::utils::file_util;
use crate::services::file_service::MultipartHandler;

#[derive(Debug, FromForm)]
pub struct UploadForm<'a> {
    somefile: TempFile<'a>,
}

#[post("/", data = "<form_data>")]
pub async fn upload_file(
    content_type: &ContentType,
    form_data: Data<'_>,
) -> Result<status::Custom<Value>, status::Custom<Value>> {
    let initial_time = time::Instant::now();

    let multipart = MultipartHandler::from(content_type, form_data)
        .await
        .map_err(|e| {
            let message =
                json!({"success": false, "message": format!("Upload Failed with error: {:#?}", e)});
            return status::Custom(Status::BadRequest, message);
        })?;

    let file_data = multipart.save_to_file().await.unwrap();

    let elapsed = initial_time.elapsed();
    let message = json!({"success": true, "message": "Upload Successful", "data": file_data, "elapsed": {"value": elapsed.as_millis() as u32, "unit": "milliseconds"}});

    Ok(status::Custom(Status::Ok, message))
}

#[get("/<filename>")]
pub async fn download_file(filename: &str) -> Result<DownloadResponse, Status> {
    let file = format!("{}/{}", file_util::STORAGE_DIRECTORY, filename);
    let path = std::path::Path::new(&file);
    DownloadResponse::from_file(path, None::<String>, None)
        .await
        .map_err(|err| {
            if err.kind() == ErrorKind::NotFound {
                Status::NotFound
            } else {
                Status::InternalServerError
            }
        })
}
