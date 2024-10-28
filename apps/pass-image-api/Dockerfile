# syntax=docker/dockerfile:1

# 
# Build the app. The base image we will use 
# will be the same as the platform we are running on.
# We will then x-compile to the TARGETARCH 
#
FROM --platform=${BUILDPLATFORM} rust:1.82-slim-bookworm AS builder

# Set working directory
WORKDIR /app

#
# Install building deps
# 
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl-dev \
    pkgconf \
    gcc-aarch64-linux-gnu \
    gcc-x86-64-linux-gnu \
    libc6-dev-arm64-cross \
    libc6-dev-amd64-cross \
    && rm -rf /var/lib/apt/lists/

ARG TARGETPLATFORM

# Linker incantation
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc    
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc

# Fetch and build the deps. Put in a stubbed main so that 
# we build everything and cache it
COPY Cargo.toml Cargo.lock ./
COPY scripts scripts
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN . scripts/target.sh && rustup target add $RUST_TARGET
RUN . scripts/target.sh && echo RUST_TARGET: $RUST_TARGET && cargo fetch --target $RUST_TARGET
RUN . scripts/target.sh && cargo build --release --target $RUST_TARGET
RUN rm src/main.rs


# Build the app itself. Make sure to touch main.rs so that we don't
# cache the results of our stub build
COPY src ./src/
RUN . scripts/target.sh && touch src/main.rs && cargo build --release --target $RUST_TARGET && cp target/$RUST_TARGET/release/pass-image-api target/pass-image-api

#
# Runtime image.
# Here we _need_ to work from the target platform, image, because
# that's where we're going to run. 
#
FROM --platform=${TARGETARCH:-$BUILDPLATFORM} debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/

# Set working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/pass-image-api .

# Expose the application's port
EXPOSE 8000

# Set the default command
CMD ["./pass-image-api"]
