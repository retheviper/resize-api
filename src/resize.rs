use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use actix_web::{HttpRequest, HttpResponse, Responder, web};
use image::{DynamicImage, ExtendedColorType};
use image::codecs::webp::WebPEncoder;
use log::{error, info, warn};
use reqwest::Client;

use crate::config::RESIZE_IMAGE_EXTENSIONS;
use crate::validate::{validate_image_url, validate_origin, validate_width};

pub(crate) async fn resize_image(req: HttpRequest, query: web::Query<HashMap<String, String>>) -> impl Responder {
    let validate = validate_origin(req);
    if !validate {
        return HttpResponse::Forbidden().body("Invalid origin");
    }

    let start = Instant::now();
    let image_url = match query.get("image_url") {
        Some(url) => match validate_image_url(url) {
            Ok(_) => format!("{}{}", *crate::config::MOUNT_PATH, url),
            Err(e) => return HttpResponse::BadRequest().body(e),
        },
        None => return HttpResponse::BadRequest().body("image_url is required"),
    };

    let width: u32 = match query.get("width") {
        Some(w) => match validate_width(w) {
            Ok(_) => w.parse().unwrap(),
            Err(e) => return HttpResponse::BadRequest().body(e),
        }
        None => return HttpResponse::BadRequest().body("width is required"),
    };

    info!("Loading image data from URL: {}", image_url);
    let img_data = match read_image_data(&image_url).await {
        Ok(img_data) => img_data,
        Err(e) => return HttpResponse::InternalServerError().body(e)
    };
    info!("Image data loaded successfully; elapsed time: {:.2}ms", start.elapsed().as_millis());

    let extension: &str = image_url.split('.').last().unwrap_or("");
    let should_resize = RESIZE_IMAGE_EXTENSIONS.contains(&extension);
    if !should_resize {
        info!("Image is not resized because it's not included target extensions. current extension: {}", extension);
        return create_image_response(&format!("image/{}", extension), img_data);
    }

    let img = match image::load_from_memory(&img_data) {
        Ok(img) => img,
        Err(e) => {
            error!("Failed to handle image: {}", e);
            return HttpResponse::InternalServerError().body("Failed to handle image");
        }
    };

    let image_width = img.width();
    if image_width <= width {
        warn!("Image width is smaller than requested width; image width: {}, requested width: {}", image_width, width);
        return create_image_response(&format!("image/{}", extension), img_data);
    }

    info!("Resizing image to width: {}", width);
    match web::block(move || resize_image_to_webp(img, width)).await {
        Ok(Ok(data)) => {
            info!("Image processed successfully; total elapsed time: {:.2}ms", start.elapsed().as_millis());
            create_image_response("image/webp", data)
        }
        Ok(Err(e)) => {
            error!("Image processing failed: {}", e);
            HttpResponse::InternalServerError().body("Image processing failed")
        }
        Err(e) => {
            error!("Blocking task failed: {}", e);
            HttpResponse::InternalServerError().body("Blocking task failed")
        }
    }
}

fn create_image_response(content_type: &str, data: Vec<u8>) -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "*"))
        .append_header(
            (
                "Cache-Control",
                format!("public, max-age={}, s-maxage={}", *crate::config::MAX_AGE, *crate::config::S_MAX_AGE)
            )
        )
        .content_type(content_type)
        .body(data)
}

async fn read_image_data(image_url: &str) -> Result<Vec<u8>, String> {
    match image_url.starts_with("http://") || image_url.starts_with("https://") {
        true => read_image_data_from_url(image_url).await,
        false => read_image_data_from_storage(image_url),
    }
}

async fn read_image_data_from_url(image_url: &str) -> Result<Vec<u8>, String> {
    let client = Client::new();

    let resp = match client.get(image_url).send().await {
        Ok(resp) => resp,
        Err(e) => {
            error!("Failed to download image: {}", e);
            return Err(format!("Failed to download image: {}", e));
        }
    };

    resp.bytes()
        .await
        .map_err(|e| format!("Failed to read image from URL: {}", e))
        .map(|b| b.to_vec())
}

fn read_image_data_from_storage(image_url: &str) -> Result<Vec<u8>, String> {
    fs::read(image_url)
        .map_err(|e| format!("Failed to read image from storage: {}", e))
}

fn resize_image_to_webp(img: DynamicImage, width: u32) -> Result<Vec<u8>, String> {
    let new_width = img.width().min(width);
    let new_height = img.height() * new_width / img.width();

    let img = img.thumbnail_exact(new_width, new_height);
    let rgb_image = img.to_rgba8();

    let mut buf = Vec::new();
    return match WebPEncoder::new_lossless(&mut buf).encode(&rgb_image, new_width, new_height, ExtendedColorType::Rgba8) {
        Ok(_) => Ok(buf),
        Err(e) => {
            error!("Failed to encode image to webp: {}", e);
            Err(format!("Failed to encode image to webp: {}", e))
        }
    };
}
