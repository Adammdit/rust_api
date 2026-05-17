# ============================
# 1. Builder image
# ============================
FROM rust:1.80 AS builder

# Install system dependencies needed for OpenSSL, pkg-config, and native builds
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy Cargo.toml and Cargo.lock first (for caching)
COPY Cargo.toml Cargo.lock ./

# Create a dummy src/main.rs to force dependency build caching
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies only
RUN cargo build --release || true

# Now copy the real source
COPY . .

# Build the actual application
RUN cargo build --release

# ============================
# 2. Runtime image
# ============================
FROM debian:bookworm-slim

# Install runtime OpenSSL
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/rust_api /app/rust_api

# Expose port (Actix default)
EXPOSE 8080

# Run the app
CMD ["/app/rust_api"]

