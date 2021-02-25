FROM rust:1.50.0

WORKDIR /app
RUN cargo install cargo-watch

COPY . .
