# Use the official Rust base image
FROM rust:latest

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

COPY .env ./

# Build the application
# RUN cargo build --release

# Set the entry point to run the application
CMD cargo run
