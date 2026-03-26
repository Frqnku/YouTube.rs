# ------- 1. CHEF STAGE ------- #
FROM messense/rust-musl-cross:x86_64-musl as chef
# Set SQLX offline mode for building without database connection
ENV SQLX_OFFLINE=true

RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs

RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add wasm32-unknown-unknown

RUN cargo install cargo-chef
# RUN cargo install --locked cargo-leptos
RUN cargo install cargo-binstall
RUN cargo binstall cargo-leptos -y
# Install sqlx-cli for database migrations
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
# Ensure .sqlx directory is copied (for SQLx offline mode)
COPY ./.sqlx /app/.sqlx
RUN npm install

ENV LEPTOS_BIN_TARGET_TRIPLE=x86_64-unknown-linux-musl
RUN cargo leptos build --release


# ------- 4. CLEANER STAGE ------- #
FROM scratch

COPY --from=builder --chmod=755 /app/target/x86_64-unknown-linux-musl/release/server /server
COPY --from=builder /app/target/site/ /site
COPY --from=builder --chmod=444 /app/public /public
# Copy migrations for potential runtime use
COPY --from=builder /app/migrations/ /migrations

WORKDIR /

ENV RUST_BACKTRACE=1
ENV LEPTOS_OUTPUT_NAME="youtube"
ENV LEPTOS_SITE_ROOT="/site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_ASSETS_DIR="public"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV SQLX_OFFLINE=true

EXPOSE 3000

CMD [ "/server" ]