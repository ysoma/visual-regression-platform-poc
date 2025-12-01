FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path services/diff-service

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/diff-service /usr/local/bin/diff-service
CMD ["diff-service"]
