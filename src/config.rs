use dotenv::dotenv;
use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref PORT: u16 = {
        dotenv().ok();
        std::env::var("PORT").expect("PORT must be set").parse().expect("PORT must be a number")
    };
    pub(crate) static ref MOUNT_PATH: String = {
        dotenv().ok();
        std::env::var("MOUNT_PATH").unwrap_or_else(|_| "".to_string())
    };
    pub(crate) static ref ENVIRONMENT: String = {
        dotenv().ok();
        std::env::var("ENVIRONMENT").unwrap_or_else(|_| "dev".to_string())
    };
    pub(crate) static ref IS_DEVELOPMENT: bool = {
        dotenv().ok();
        std::env::var("ENVIRONMENT").unwrap_or_else(|_| "dev".to_string()) == "dev"
    };
    pub(crate) static ref MAX_AGE: u32 = {
        dotenv().ok();
        std::env::var("MAX_AGE").unwrap_or_else(|_| (60 * 60 * 24).to_string()).parse().expect("MAX_AGE must be a number")
    };
    pub(crate) static ref S_MAX_AGE: u32 = {
        dotenv().ok();
        std::env::var("S_MAX_AGE").unwrap_or_else(|_| (60 * 60 * 24 * 30).to_string()).parse().expect("S_MAX_AGE must be a number")
    };
    pub(crate) static ref ALLOWED_ORIGINS: Vec<String> = {
        dotenv().ok();
        let allowed_origins_str = std::env::var("ALLOWED_ORIGINS").expect("ALLOWED_ORIGINS must be set");
        allowed_origins_str.split(',').map(|s| s.to_string()).collect()
    };
}

pub(crate) const IMAGE_EXTENSIONS: [&str; 5] = ["jpg", "jpeg", "png", "webp", "gif"];

pub(crate) const RESIZE_IMAGE_EXTENSIONS: [&str; 3] = ["jpg", "jpeg", "png"];