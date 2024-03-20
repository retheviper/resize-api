# Image resize API

## TL;DR

A simple API that just resizes images and returns as webp.

## Description

Actix-web based API that resizes images.
Using the `image` crate, the API downloads the image from the provided URL, resizes it to the specified width, and returns the resized image.
Set `.env` file with the following environment variables:
- `RUST_LOG`: The log level for the application
- `PORT`: The port on which the server will run
- `MOUNT_PATH`: The path to mount (where images will be stored)
- `ENVIRONMENT`: The environment in which the application is running
  - `dev`: Development environment (default, allows url to be any)
- `MAX_AGE`: The maximum age of the cache
- `S_MAX_AGE`: The maximum age of the cache for shared caches
- `ALLOWED_ORIGINS`: The allowed origins for CORS

## Supported image formats (to be resized)

- JPEG
- PNG

Or any other format will be just returned as is.

## How to run

1. Clone the repository
2. Run `cargo run` in the root directory
   1. Or `docker compose up` to run the application in a docker container
3. The server will start at `http://localhost:8100`
4. Send a GET request to `http://localhost:8100/v1/images/resize` with the following Query Parameters:
    - `image_url`: The URL of the image to resize
    - `width`: The width of the resized image (in pixels)

Image's height will be calculated based on the aspect ratio of the original image.