# ------------------------------
# Stage 1. Build an app
# ------------------------------
FROM rust:1.96.0 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

# ------------------------------
# Stage 2. Build for runtime
# ------------------------------
FROM dhi.io/debian-base:trixie

ARG GIT_REVISION
ARG BUILD_DATE
ARG VERSION

LABEL org.opencontainers.image.title="img-ascii" \
      org.opencontainers.image.description="Convert images into ASCII art" \
      org.opencontainers.image.url="https://yudaitakeda.github.io/img-ascii/" \
      org.opencontainers.image.source="https://github.com/yudaitakeda/img-ascii" \
      org.opencontainers.image.version=${VERSION} \
      org.opencontainers.image.revision=${GIT_REVISION} \
      org.opencontainers.image.created=${BUILD_DATE} \
      org.opencontainers.image.licenses="MIT"

COPY --from=builder /app/target/release/img-ascii /app/img-ascii

WORKDIR /app

ENTRYPOINT ["/app/img-ascii"]
