FROM rust:1.76.0-alpine3.19

RUN apk add musl-dev

WORKDIR /app

ADD . /app

env SQLX_OFFLINE true
RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/rinha-de-backend"]
