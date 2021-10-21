use crate::models::file::FileData;
use rocket::data::Data;
use rocket::http::{ContentType, Status};
use rocket::response::status;
use rocket_multipart_form_data::{
    mime::Mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

use async_std::io::prelude::*;

use crate::handlers::error::TransmissionError;
use crate::utils::file_util::{FileUtil, STORAGE_DIRECTORY};

pub struct MultipartHandler {
    pub content_type: Option<Mime>,
    pub file_name: String,
    pub raw: Vec<u8>,
}

impl MultipartHandler {
    pub async fn from(
        content_type: &ContentType,
        form_data: Data<'_>,
    ) -> Result<Self, TransmissionError> {
        let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
            MultipartFormDataField::raw("somefile").size_limit(200 * 1024 * 1024),
        ]);
        let mut multipart_form_data =
            MultipartFormData::parse(&content_type, form_data, options).await?;
        // .map_err(|e| e.into())?;
        // .map_err(|e| TransmissionError::Message("Failed to parse"))?;

        let content = multipart_form_data
            .raw
            .remove("somefile")
            .ok_or_else(|| TransmissionError::Message("No data found in file".to_string()))?;

        let file_name = content[0]
            .file_name
            .clone()
            .ok_or_else(|| TransmissionError::Message("Could not get filename".to_string()))?;

        Ok(Self {
            content_type: content[0].content_type.clone(),
            file_name: file_name,
            raw: content[0].raw.clone(),
        })
    }

    pub async fn save_to_file(&self) -> Result<FileData, TransmissionError> {
        let path = async_std::path::Path::new(STORAGE_DIRECTORY);
        if !path.exists().await {
            async_std::fs::create_dir(path).await?;
            // .map_err(|e| e.into());
            // .map_err(|e| TransmissionError::Message("Failed to save data to file"))?;
        }

        let mut file =
            async_std::fs::File::create(format!("{}/{}", STORAGE_DIRECTORY, self.file_name))
                .await?;

        // .map_err(|e| e.into())?;
        // .map_err(|e| TransmissionError::Message("Failed to save data to file"))?;

        file.write_all(&self.raw).await?;
        // .map_err(|e| e.into());
        // .map_err(|e| TransmissionError::Message("Failed to save data to file"))?;
        file.sync_all().await?;
        let meta_data = file.metadata().await?;

        let file = FileData {
            name: self.file_name.to_owned(),
            url: format!("{}/{}", FileUtil::get_basefile_path().await, self.file_name),
            size: meta_data.len(),
            size_unit: "bytes".to_owned()
        };

        Ok(file)
    }
}
