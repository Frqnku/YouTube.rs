# ------- 1. CHEF STAGE ------- #
FROM messense/rust-musl-cross:x86_64-musl as chef

ENV SQLX_OFFLINE=true

# System deps for Rust + SQLx + MUSL
RUN apt-get update && apt-get install -y \
    curl \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs
RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add wasm32-unknown-unknown

RUN cargo install cargo-chef
RUN cargo install cargo-binstall
RUN cargo binstall cargo-leptos -y
RUN cargo install sqlx-cli --no-default-features --features postgres

WORKDIR /app

# ------- 2. PLANNER STAGE ------- #
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ------- 3. BUILDER STAGE ------- #
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY . .
COPY ./.sqlx /app/.sqlx
RUN npm install
ENV LEPTOS_BIN_TARGET_TRIPLE=x86_64-unknown-linux-musl
RUN cargo leptos build --release

# ------- 4. RUNTIME STAGE ------- #
FROM debian:bookworm-slim AS runtime

# Add ca-certificates for HTTPS and PostgreSQL client for migrations if needed
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy compiled binary and migrations
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/server ./server
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/scripts/deploy/setup_env.sh ./scripts/deploy/setup_env.sh
COPY --from=builder /app/target/site /site

# Ensure binary is executable
RUN chmod +x ./server

# Expose port if needed (example 8080)
EXPOSE 8080

# Entrypoint
ENTRYPOINT ["/app/server"]