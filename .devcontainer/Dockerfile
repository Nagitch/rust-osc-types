FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# Install additional tooling here if needed. For now we just ensure the apt cache is clean.
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config \
    && rm -rf /var/lib/apt/lists/*

ENV CARGO_TERM_COLOR=always
