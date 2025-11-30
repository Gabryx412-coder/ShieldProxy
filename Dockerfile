# Stage 1: Chef (gestisce il caching delle dipendenze)
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# Stage 2: Planner
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Builder
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build delle sole dipendenze per caching
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
# Build del binario reale
RUN cargo build --release --bin shield-proxy

# Stage 4: Runtime
FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /app/target/release/shield-proxy /app/shield-proxy
# Copia config di default (opzionale, meglio montare volume)
COPY config/ /app/config/

ENV RUST_LOG=info
EXPOSE 8443 9090

ENTRYPOINT ["./shield-proxy"]