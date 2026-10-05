FROM rust:1.82 AS builder
WORKDIR /app
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ffmpeg ca-certificates fonts-inter \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/murmur-clipper /usr/local/bin/murmur-clipper
EXPOSE 8080
CMD ["murmur-clipper"]
