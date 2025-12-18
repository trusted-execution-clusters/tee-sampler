# rust:1.89.0-slim
FROM --platform=${BUILDPLATFORM:-linux/amd64} \
    docker.io/library/rust@sha256:6c828d9865870a3bc8c02919d73803df22cac59b583d8f2cb30a296abe64748f \
    AS builder
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    curl \
    gpg \
    gnupg-agent \
    git \
    sudo

RUN curl -fsSL https://download.01.org/intel-sgx/sgx_repo/ubuntu/intel-sgx-deb.key | \
    gpg --dearmor --output /usr/share/keyrings/intel-sgx.gpg && \
    echo 'deb [arch=amd64 signed-by=/usr/share/keyrings/intel-sgx.gpg] https://download.01.org/intel-sgx/sgx_repo/ubuntu jammy main' | \
    tee /etc/apt/sources.list.d/intel-sgx.list && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
    libclang-dev \
    libprotobuf-dev \
    libssl-dev \
    make \
    perl \
    pkg-config \
    protobuf-compiler \
    wget \
    clang \
    cmake \
    libtss2-dev \
    libsgx-dcap-quote-verify-dev


WORKDIR /app

# Copy source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the project in release mode
RUN cargo build --release

# Runtime stage
FROM fedora:latest

WORKDIR /app

RUN dnf install -y \
    ca-certificates \
    openssl \
    tpm2-tss \
    sgx-libs \
    tdx-attest-libs \
    libstdc++ \
    zlib \
    zstd \
    && dnf clean all && \
    ldconfig

# Copy the binary from builder
COPY --from=builder /app/target/release/hardware-sampler /app/hardware-sampler

# Make the binary executable
RUN chmod +x /app/hardware-sampler

# Set the binary as the entrypoint
ENTRYPOINT ["/app/hardware-sampler"]