FROM rust:latest

RUN cargo install cargo-watch

CMD ["cargo", "watch", "-x", "run"]
