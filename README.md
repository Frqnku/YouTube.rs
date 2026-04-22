# YouTube.rs - Fullstack Rust Systems Exploration

Production-style fullstack Rust platform focused on architecture, performance, and systems design for media-heavy workloads.
Demo: https://ytb-frqnku.live

## Motivation

This project was built as an engineering exploration, not a product-first exercise.

The goals were to:

- Evaluate how far Rust can go across the full stack (backend + SSR frontend + deployment).
- Design a layered architecture with explicit boundaries between domain, application, and infrastructure concerns.
- Exercise async/concurrency patterns around user activity, watch progress, and server-rendered interactions.
- Understand tradeoffs involved in building a video-centric web platform in Rust.

## System Overview

The codebase is organized as a Rust workspace with clear separation of concerns:

- `domain`: core business model and contracts (entities, value objects, repository traits).
- `application`: use cases split into commands and queries, plus DTO mapping.
- `infrastructure`: SQLx/PostgreSQL repositories, JWT token service, Google OAuth service.
- `interface/web`: Leptos UI, routes, and typed server functions.
- `interface/server`: Axum bootstrap, SSR rendering, middleware, app state wiring.

Request flow (high-level):

1. Axum receives HTTP requests and enriches request context (client IP, authenticated user from JWT cookie).
2. Leptos SSR handlers and server functions resolve use cases from `application`.
3. Use cases depend on repository/service traits from `domain`/`application`.
4. Concrete implementations in `infrastructure` execute SQLx queries and external integrations.
5. Results are returned as typed DTOs to Leptos components.

## Tech Stack

### Backend

- Rust (nightly), Tokio (multi-thread runtime)
- Axum + `leptos_axum` for HTTP + SSR integration
- SQLx (PostgreSQL, offline query metadata support)
- `tracing` / `tracing-subscriber` for structured logs
- JWT auth (`jsonwebtoken`)
- OAuth integration (`oauth2` + Google OpenID userinfo endpoint)

### Frontend

- Leptos (SSR + hydration)
- Leptos Router / Leptos Meta
- WASM target: `wasm32-unknown-unknown`
- TailwindCSS (CLI)

### Infrastructure / Ops

- PostgreSQL 15
- AWS S3 for video object storage + CloudFront as edge cache/CDN
- Docker multi-stage build (cargo-chef + musl target)
- Docker Compose (app + db + nginx)
- Nginx reverse proxy + TLS termination setup
- GitHub Actions workflow for build/push/deploy (`.github/workflows/prod.yml`)

## Key Engineering Highlights

- Layered architecture with dependency direction from outer layers toward stable core abstractions.
- Command/query split in application services improves testability and keeps mutation/read paths explicit.
- Cursor-based pagination for video and comment feeds (newest/popular ordering) using stable cursor tuples.
- View counting logic deduplicates by user and/or IP with configurable recount windows and transactional updates.
- Watch progress persistence combines immediate event-based updates with periodic async sync while video is playing.
- Media delivery uses S3-backed video assets fronted by CloudFront to reduce origin load and improve global cache hit performance.
- Context propagation from Axum middleware into Leptos server functions enables SSR-safe auth and client metadata access.
- Domain value objects (`Email`, `Url`, etc.) enforce invariants early, reducing invalid state in deeper layers.
- SQL schema is migration-driven and index-aware for high-frequency access paths (feeds, reactions, history, comments).

## Database Modeling

Core tables include:

- `users`, `user_oauth_providers`
- `videos`, `video_views`, `video_reactions`
- `video_comments`, `comment_likes`
- `tags`, `video_tags`
- `subscriptions`, `channels`

Modeling choices emphasize:

- relational integrity via foreign keys/cascades,
- replay-safe interaction tracking (views/reactions),
- indexed read paths for timeline/search/history workloads,
- denormalized counters (`view_count`, `like_count`, `subscriber_count`, `video_count`) for fast read models.

## Project Structure

```text
.
├─ Cargo.toml                    # Workspace definition
├─ migrations/                   # SQL schema and seed data
├─ src/
│  ├─ domain/                    # Entities, value objects, repository traits
│  ├─ application/               # Commands, queries, DTOs, tests
│  ├─ infrastructure/            # Postgres repositories + infra services
│  └─ interface/
│     ├─ server/                 # Axum entrypoint, middleware, SSR router
│     └─ web/                    # Leptos UI, routes, server functions
├─ deploy/nginx/                 # Nginx production config
├─ scripts/                      # DB reset, server bootstrap, deploy helpers
├─ docker-compose.yml            # App + DB + Nginx topology
└─ Dockerfile                    # Multi-stage release build
```

## Getting Started (Local)

### Prerequisites

- Rust nightly toolchain
- Target: `wasm32-unknown-unknown`
- `cargo-leptos`
- Node.js (for Tailwind CLI)
- PostgreSQL
- `sqlx-cli` (for migrations)

### 1) Install dependencies

```bash
rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown

cargo install cargo-binstall
cargo binstall cargo-leptos -y
cargo install sqlx-cli --no-default-features --features postgres

npm install
```

### 2) Configure environment

Create a `.env` at project root with at least:

```env
DATABASE_URL=postgres://<user>:<password>@127.0.0.1:5432/<database>
JWT_SECRET=<strong-secret>

POSTGRES_USER=<user>
POSTGRES_PASSWORD=<password>
POSTGRES_DATABASE=<database>

GOOGLE_CLIENT_ID=<google-client-id>
GOOGLE_CLIENT_SECRET=<google-client-secret>
OAUTH_GOOGLE_REDIRECT_URI=http://localhost:3000/signin
```

You will also need to configure an application for Google OAuth

### 3) Create schema

```bash
sqlx migrate run
```

Optional reset:

```bash
bash ./scripts/reset_db.sh
```

### 4) Run in development

In one terminal:

```bash
npm run tailwind
```

In another terminal:

```bash
cargo leptos watch
```

App default bind address in workspace metadata: `0.0.0.0:3000`.

## Containerized / Deployment Notes

- Dockerfile uses `cargo-chef` for dependency layer caching and builds a musl-targeted release binary.
- Runtime image ships server binary, migrations, and generated site assets.
- Compose topology: app + Postgres + Nginx.
- GitHub Actions workflow builds/pushes image, deploys via SSH, and supports optional DB volume reset.

## What I Learned

- Rust can support end-to-end fullstack development with strong type safety across domain, API contracts, and UI boundaries.
- A strict domain/application/infrastructure split keeps growth manageable even when feature count increases.
- SSR + server functions in Leptos provide a productive model, but context wiring and hydration timing require careful handling.
- SQL-first modeling with targeted indexes and counters is critical for responsive feed-style user experiences.
- Async runtime choices are less about raw throughput and more about predictable behavior under mixed IO workloads.

## Closing

This project demonstrates how I approach fullstack Rust engineering as a systems problem: explicit architecture, measurable tradeoffs, and production-oriented implementation choices.
