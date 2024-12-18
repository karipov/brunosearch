# frontend build stage
FROM oven/bun:1-alpine AS frontend
WORKDIR /app
COPY frontend/package.json frontend/bun.lockb ./
RUN bun install
COPY frontend/ .
RUN bun run build

# backend build stage (cache dependencies)
FROM rust:1-bookworm AS backend
WORKDIR /app
COPY backend/Cargo.toml backend/Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch
RUN cargo build --release
RUN rm src/main.rs
COPY backend/src ./src/
RUN touch src/main.rs
RUN cargo build --release

# redis-stack build stage
FROM redis/redis-stack-server:7.4.0-v1 AS redis-stack

# final stage
FROM redis:7.4-bookworm
RUN ln -sf /bin/bash /bin/sh
RUN apt-get update && apt-get install -y ca-certificates procps && apt-get clean
COPY --from=redis-stack /opt/redis-stack/lib/redisearch.so /opt/redis-stack/lib/redisearch.so
COPY --from=redis-stack /opt/redis-stack/lib/rejson.so /opt/redis-stack/lib/rejson.so
COPY --from=redis-stack /opt/redis-stack/lib/redisbloom.so /opt/redis-stack/lib/redisbloom.so
COPY --from=backend /app/target/release/backend /usr/bin
COPY --from=frontend /app/dist static
COPY data data
CMD backend --frontend static --reindex --courses data/spring2025/courses.json --embedded data/spring2025/courses_embedded.json
