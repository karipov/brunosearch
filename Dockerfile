# frontend build stage
FROM oven/bun:1-alpine AS frontend
WORKDIR /app
COPY frontend/package.json frontend/bun.lockb ./
RUN bun install
COPY frontend/ .
RUN bun run build

# backend build stage (cache dependencies)
FROM rust:1-alpine AS backend
WORKDIR /app
COPY backend/Cargo.toml backend/Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release
RUN rm -rf src
COPY backend/ .
RUN cargo build --release

# redis-stack build stage
FROM redis/redis-stack-server:7.4.0-v1 AS redis-stack

# final stage
FROM redis:7.4-alpine
WORKDIR /app
RUN apk update && apk add --no-cache ca-certificates
COPY --from=redis-stack /opt/redis-stack/lib/redisearch.so /opt/redis-stack/lib/redisearch.so
COPY --from=redis-stack /opt/redis-stack/lib/rejson.so /opt/redis-stack/lib/rejson.so
COPY --from=backend /app/target/release/backend /usr/local/bin/backend
COPY --from=frontend /app/dist static
CMD ["backend"]
