FROM rust:1.70-alpine AS builder
WORKDIR /usr/src/app
RUN apk add --no-cache musl-dev
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl
FROM alpine:3.14
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/log-generator /main
CMD ["/main"]