FROM rust:latest
WORKDIR /root/nsm-cli
ARG ARCH

# Ensure the architecture is set
RUN [ -z "$ARCH" ] && echo "ARCH is required" && exit 1 || true

# RUN apt-get update && apt-get install gcc curl -y

# # Get Rust
# RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

# ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target install ${ARCH}-unknown-linux-musl

# Build the Rust application
COPY Cargo.toml ./
COPY Cargo.lock ./
COPY src ./src

RUN cargo build --release --target=${ARCH}-unknown-linux-musl
RUN strip -s target/${ARCH}-unknown-linux-musl/release/nsm-cli
