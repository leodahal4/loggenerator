FROM rust:1.70 AS builder
WORKDIR /usr/src/app
COPY . .
# RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
#RUN rm src/main.rs
#COPY src ./src
#RUN cargo build --release
FROM debian:buster-slim
# RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/log-generator ./main
CMD ["./main"]