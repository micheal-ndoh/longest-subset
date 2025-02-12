FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/longest-subset

COPY . .

# Build the Rust program
RUN cargo build --release

FROM ubuntu:22.04

# Set the working directory
WORKDIR /usr/src/longest-subset

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/longest-subset/target/release/longest-subset .

# Run the executable
CMD ["./longest-subset"]