use image::DynamicImage;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RequestResponse {
    pub duration: Option<String>,
    pub status_code: Option<String>,
    pub content: Option<ResponseContent>,
    pub cookies: Option<String>,
    pub headers: Vec<(String, String)>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseContent {
    Body(String),
    Image(ImageResponse)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageResponse {
    pub data: Vec<u8>,
    #[serde(skip)]
    pub image: Option<DynamicImage>
}
