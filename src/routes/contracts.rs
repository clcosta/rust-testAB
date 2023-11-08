use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SlugPayLoad {
    pub slug: String,
}

#[derive(Deserialize)]
pub struct SlugVariantPayLoad {
    pub slug: String,
    pub variant_name: String,
    pub url: String 
}

#[derive(Serialize)]
pub struct Data {
    pub data: std::collections::HashMap<String, String>,
}

impl Data {
    pub fn new(data: std::collections::HashMap<String, String>) -> Self {
        Self {
            data
        }
    }
}

#[derive(Serialize)]
pub struct BaseResponse {
    pub status_code: u16,
    pub message: String,
    pub data: Option<Data>
}

impl BaseResponse {
    pub fn new(status_code: u16, message: String, data: Option<Data>) -> Self {
        Self {
            status_code,
            message,
            data
        }
    }
}