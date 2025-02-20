# Use the official Rust image as a base
FROM rust:1.75 AS builder

# Set the working directory
WORKDIR /app

# Copy the Cargo manifest and lock file separately to leverage Docker's caching
COPY Cargo.toml ./

# Create a blank binary crate to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Fetch and compile dependencies
RUN cargo build --release && rm -rf src

# Copy the actual source code and build the final binary
COPY . .
RUN cargo build --release

# Use a minimal runtime image
FROM debian:bullseye-slim

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/simple-alert /app/simple-alert

# Expose the application's port
EXPOSE 3000

# Run the application
CMD ["/app/simple-alert"]
