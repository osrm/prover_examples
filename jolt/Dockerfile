# Stage 1: Build the application
FROM --platform=linux/amd64 ubuntu:jammy AS builder

ENV PATH="/root/.cargo/bin:$PATH"

# Install dependencies for building
RUN apt-get update \
    && apt-get install -y curl build-essential libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Install rustup and the necessary toolchain
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y
RUN rustup toolchain install nightly-2024-10-01 --profile minimal

# Install Jolt toolchain
RUN cargo +nightly-2024-10-01 install --git https://github.com/a16z/jolt.git --rev 6dac58ca1c92621ee509354d9bc0f6f023204016 --force --bins jolt
RUN jolt install-toolchain

# Set the working directory
WORKDIR /app

# Copy and build the source code
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY rust-toolchain.toml rust-toolchain.toml
COPY guest guest
COPY bin bin
RUN cargo build --release


# Stage 2: Create the final slim image
FROM --platform=linux/amd64 ubuntu:jammy

# Install only the necessary runtime dependencies
RUN apt-get update \
    && apt-get install -y curl libssl-dev build-essential \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

ENV PATH="/root/.cargo/bin:$PATH"
    
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y

RUN rustup toolchain install nightly-2024-10-01 --profile minimal \
    && rustup component add rustc cargo

WORKDIR /app

ENV PROOF_LOCATION=/output/proof.bin \
    A= \
    B= \
    PATH="/root/.cargo/bin:$PATH"

# Copy the built binaries from the builder stage
COPY --from=builder /app/target/release/prove prove
COPY --from=builder /app/target/release/verify verify
COPY --from=builder /app/guest guest
COPY --from=builder /app/Cargo.toml Cargo.toml
COPY --from=builder /app/Cargo.lock Cargo.lock
COPY --from=builder /app/rust-toolchain.toml rust-toolchain.toml
COPY --from=builder /root/.jolt/.toolchaintag /root/.jolt/.toolchaintag
COPY --from=builder /root/.jolt/rust /root/.jolt/rust

# Set the entry point
ENTRYPOINT ["/app/prove"]