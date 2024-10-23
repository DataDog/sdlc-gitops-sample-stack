# syntax=docker/dockerfile:1

# Set the default target architecture to BUILDPLATFORM's architecture if TARGETARCH is not set
# ARG TARGETARCH
# ARG BUILDPLATFORM

# 
# Build the app 
#
FROM --platform=${TARGETARCH:-$BUILDPLATFORM} rust:1.82-slim-bookworm AS builder

# Set working directory
WORKDIR /app

#
# Install building deps
# 
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl-dev \
    pkgconf \
    && rm -rf /var/lib/apt/lists/

# Fetch and build the deps. Put in a stubbed main so that 
# we build everything and cache it
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch
RUN cargo build --release
RUN rm src/main.rs


# Build the app itself 
COPY src ./src/
RUN cargo build --release

#
# Runtime image 
#
FROM --platform=${TARGETARCH:-$BUILDPLATFORM} debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/

# Set working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/pass-image-api .

# Expose the application's port
EXPOSE 8000

# Set the default command
CMD ["./pass-image-api"]
