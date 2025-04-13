FROM rust:1.85 AS builder
WORKDIR /usr/src/rocket-backend-template
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rocket-backend-template /usr/local/bin/rocket-backend-template

EXPOSE 3000

CMD ["rocket-backend-template"] 