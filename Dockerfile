# Stage 1: Build the Rust application
FROM rust:latest AS builder

WORKDIR /usr/src/rust_send_discord
COPY . .

# Build the application in release mode
RUN cargo build --release

# Debug: List the files in the target/release directory
RUN ls -l /usr/src/rust_send_discord/target/release/

# Stage 2: Create a smaller runtime image
FROM ubuntu:latest

# Install necessary dependencies to run the Rust binary
RUN apt-get update && apt-get install -y \
    libssl-dev \
    libpq-dev \
    libc6 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/rust_send_discord/target/release/rust_send_discord /usr/local/bin/rust_send_discord

# Set the default command to run the application
CMD ["rust_send_discord"]
