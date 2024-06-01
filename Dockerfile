FROM rust:1.78.0  as builder

WORKDIR /app

RUN apt-get update && apt-get install -y musl-tools

COPY . .

ENV SQLX_OFFLINE true

RUN cargo build --release --target x86_64-unknown-linux-musl

ENTRYPOINT ["cargo", "run", "--release", "--target", "x86_64-unknown-linux-musl"]