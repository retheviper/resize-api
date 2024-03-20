use actix_web::HttpRequest;
use crate::config::{ALLOWED_ORIGINS, IS_DEVELOPMENT};

pub(crate) enum UrlType {
    Http,
    Local,
}

pub(crate) fn validate_origin(req: HttpRequest) -> bool {
    if *IS_DEVELOPMENT {
        return true;
    }

    let origin = req.headers().get("Origin").and_then(|o| o.to_str().ok()).unwrap_or("");
    ALLOWED_ORIGINS.contains(&origin.to_string())
}

pub(crate) fn validate_image_url(image_url: &str) -> Result<UrlType, String> {
    if image_url.is_empty() {
        return Err("image_url is required.".to_string());
    }

    let extension: &str = image_url.split('.').last().unwrap_or("");
    let is_image_extension: bool = crate::config::IMAGE_EXTENSIONS.contains(&extension);
    if !is_image_extension {
        return Err(format!("{} is not image extension.", extension));
    }

    let is_http = image_url.starts_with("http://") || image_url.starts_with("https://");

    if is_http && !*IS_DEVELOPMENT {
        return Err("image_url is invalid.".to_string());
    }

    Ok(if is_http {
        UrlType::Http
    } else {
        UrlType::Local
    })
}

pub(crate) fn validate_width(width: &str) -> Result<u32, String> {
    if width.is_empty() {
        return Err("width is required.".to_string());
    }

    let width = width.parse::<i32>();

    match width {
        Ok(w) => {
            if w <= 0 {
                return Err("width should be greater than 0.".to_string());
            }
        },
        Err(_) => return Err("width should be number.".to_string()),
    };

    Ok(width.unwrap() as u32)
}