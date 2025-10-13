# syntax=docker/dockerfile:1
FROM rust:1.76-slim

# Install common build tools and clean up apt caches to keep the image lean.
RUN apt-get update \
    && apt-get install -y --no-install-recommends build-essential pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace

# Pre-fetch dependencies using only the manifest files to leverage Docker layer caching.
COPY Cargo.toml Cargo.lock ./
COPY osc-types10/Cargo.toml osc-types10/
COPY osc-types11/Cargo.toml osc-types11/
RUN cargo fetch

# Copy the rest of the repository.
COPY . .

# Set the default command to run the full test suite.
CMD ["cargo", "test"]
