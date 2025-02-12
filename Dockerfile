FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/subset

COPY . .

# Build the Rust program
RUN cargo build --release

FROM ubuntu:22.04

# Set the working directory
WORKDIR /usr/src/subset

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/subset/target/release/subset .

# Run the executable
CMD ["./subset"]