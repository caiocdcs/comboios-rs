# Cargo chef for dependency caching
FROM rust:1.85-slim-bookworm AS chef
RUN cargo install cargo-chef
WORKDIR /app

# Plan stage - analyze dependencies
FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY comboios-core ./comboios-core
COPY comboios-server ./comboios-server
COPY comboios-mcp ./comboios-mcp
RUN cargo chef prepare --recipe-path recipe.json

# Build stage - compile dependencies first (cached), then app
FROM chef AS rust-builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY Cargo.toml Cargo.lock ./
COPY comboios-core ./comboios-core
COPY comboios-server ./comboios-server
COPY comboios-mcp ./comboios-mcp
RUN cargo build --release -p comboios-server

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=rust-builder /app/target/release/comboios-server /app/comboios-server

ENV HOST=0.0.0.0
ENV PORT=3000
ENV RUST_LOG=info

EXPOSE 3000

CMD ["/app/comboios-server"]
