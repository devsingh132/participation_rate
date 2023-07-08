# Use the official Rust base image
FROM rust:latest

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

COPY .env ./


# Set the entry point to run the application
CMD sleep ${DELAY} && cargo run
