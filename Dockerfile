FROM docker.io/library/node:22-bookworm-slim AS frontend
WORKDIR /app/frontend
COPY frontend/package.json frontend/package-lock.json* ./
RUN npm install
COPY frontend/ ./
RUN npm run build

FROM docker.io/library/rust:1-bookworm AS backend
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build

FROM docker.io/library/debian:bookworm-slim AS runtime
WORKDIR /app
# RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=backend /app/target/debug/parking-control /usr/local/bin/parking-control
COPY frontend/public ./frontend/public
COPY --from=frontend /app/frontend/public/assets ./frontend/public/assets
EXPOSE 8000
CMD ["parking-control"]
