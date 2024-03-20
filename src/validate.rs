use actix_web::HttpRequest;
use crate::config::{ALLOWED_ORIGINS, IS_DEVELOPMENT};

pub(crate) fn validate_origin(req: HttpRequest) -> bool {
    if *IS_DEVELOPMENT {
        return true;
    }

    let origin = req.headers().get("Origin").and_then(|o| o.to_str().ok()).unwrap_or("");
    ALLOWED_ORIGINS.contains(&origin.to_string())
}

pub(crate) fn validate_image_url(image_url: &str) -> Result<(), String> {
    if image_url.is_empty() {
        return Err("image_url is required.".to_string());
    }

    let extension: &str = image_url.split('.').last().unwrap_or("");
    let is_image_extension: bool = crate::config::IMAGE_EXTENSIONS.contains(&extension);
    if !is_image_extension {
        return Err(format!("{} is not image extension.", extension));
    }

    if (image_url.starts_with("http://") || image_url.starts_with("https://")) && !*IS_DEVELOPMENT {
        return Err("image_url is invalid.".to_string());
    }

    Ok(())
}

pub(crate) fn validate_width(width: &str) -> Result<(), String> {
    if width.is_empty() {
        return Err("width is required.".to_string());
    }

    if !width.parse::<u32>().is_ok() {
        return Err("width should be number.".to_string());
    }

    Ok(())
}