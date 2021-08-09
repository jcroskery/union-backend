FROM rust as builder
WORKDIR /usr/src/union
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/union /usr/local/bin/union
CMD ["union"]
