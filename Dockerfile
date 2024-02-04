# Builder stage
FROM rust:1.75.0 as builder

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

# Runtime stage
# FROM rust:1.75.0-slim as runtime
# Shave off the whole Rust toolchain and machinery (i.e. rustc, cargo, etc) - none of that is
# needed to RUN our binary.
# Use the bare operating system as base image for our runtime stage.
FROM debian:bookworm-slim as runtime

WORKDIR /app
# Install OpenSSL - it is tynamically liked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates 
# when stablishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod
# We need the configuration file at runtime!
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]

# We could go even smaller by using rust:1.72.0-alpine, but we would have to 
# cross-compile to the linuxmusl target - out of scope for now.
# Check out (rust-musl-builder)[https://github.com/clux/muslrust] if you are
# interested in generating tiny Docker images.
# Another option to reduce the size of our binary further is stripping symbols from 
# it - you can find more information about it 
# (here)[https://github.com/johnthagen/min-sized-rust#strip-symbols-from-binary].
