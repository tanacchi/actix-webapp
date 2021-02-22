FROM rust:latest

WORKDIR /app
RUN cargo install cargo-watch

COPY . .
