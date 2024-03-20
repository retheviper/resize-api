mod resize;
mod validate;
mod config;

use actix_web::{App, HttpServer, web};
use crate::resize::resize_image;
use crate::config::PORT;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = *PORT;
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .route("/v1/images/resize", web::get().to(resize_image))
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}