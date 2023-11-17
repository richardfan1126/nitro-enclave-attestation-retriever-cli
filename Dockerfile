FROM rust:latest
WORKDIR /root/nsm-cli
ARG ARCH

# Ensure the architecture is set
RUN [ -z "$ARCH" ] && echo "ARCH is required" && exit 1 || true

# Add build target
RUN rustup target add ${ARCH}-unknown-linux-musl

# Build the Rust application
COPY Cargo.toml ./
COPY Cargo.lock ./
COPY src ./src

RUN cargo build --release --target=${ARCH}-unknown-linux-musl
RUN strip -s target/${ARCH}-unknown-linux-musl/release/nsm-cli
