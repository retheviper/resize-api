FROM rust:latest

ARG NAME=resize-api

COPY Cargo.toml .
COPY Cargo.lock .
COPY .env .
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs && cargo build --release

COPY src src
RUN CARGO_BUILD_INCREMENTAL=true cargo build --release && cp target/release/${NAME} target/release/app

ENTRYPOINT ["./target/release/app"]
