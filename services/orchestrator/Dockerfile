FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path services/orchestrator

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/orchestrator /usr/local/bin/orchestrator
CMD ["orchestrator"]
