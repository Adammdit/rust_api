FROM rust:alpine AS builder

RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    pkgconfig \
    build-base

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:3.19

RUN adduser -D appuser

WORKDIR /app

COPY --from=builder /app/target/release/rust_api /app/rust_api

USER appuser

EXPOSE 8080

CMD ["./rust_api"]

