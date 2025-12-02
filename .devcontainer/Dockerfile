# .devcontainer/Dockerfile
FROM debian:bookworm-slim

# Install dependencies
RUN apt-get update && \
    apt-get install -y curl build-essential pkg-config libssl-dev ca-certificates git && \
    rm -rf /var/lib/apt/lists/*

# Install Rust using rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Add Rust to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Install rust-analyzer for IDE features
RUN curl -L https://github.com/rust-lang/rust-analyzer/releases/latest/download/rust-analyzer-linux \
    -o /usr/local/bin/rust-analyzer && \
    chmod +x /usr/local/bin/rust-analyzer
