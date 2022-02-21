FROM ubuntu:latest AS builder
WORKDIR /root/nitro-enclave-attestation-retriever-cli
ARG ARCH

# Ensure the architecture is set
RUN [ -z "$ARCH" ] && echo "ARCH is required" && exit 1 || true

RUN apt-get update && apt-get install gcc curl -y

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target install ${ARCH}-unknown-linux-musl

# Build the Rust application
COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release --target=${ARCH}-unknown-linux-musl



FROM amazonlinux
WORKDIR /app
ARG ARCH

# Install python for running the server
RUN yum install python3 -y

COPY example/server/requirements.txt ./
RUN pip3 install -r /app/requirements.txt

COPY example/server/server.py ./
COPY example/server/run.sh ./

RUN chmod +x run.sh

# Copy the Rust compiled binary from previous image
COPY --from=builder /root/nitro-enclave-attestation-retriever-cli/target/${ARCH}-unknown-linux-musl/release/nitro-enclave-attestation-retriever-cli .

CMD ["/app/run.sh"]
