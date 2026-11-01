FROM docker.io/library/node:22-bookworm-slim AS frontend-build
WORKDIR /app/frontend
COPY frontend/package.json frontend/package-lock.json* ./
RUN npm install
COPY frontend/ ./
RUN npm run build

FROM docker.io/library/rust:1-bookworm AS backend-build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
ARG CARGO_BUILD_JOBS
RUN if [ -n "$CARGO_BUILD_JOBS" ]; then \
      cargo build --release --jobs "$CARGO_BUILD_JOBS"; \
    else \
      cargo build --release; \
    fi

FROM docker.io/library/debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=backend-build /app/target/release/parking-control /usr/local/bin/parking-control
COPY frontend/public ./frontend/public
COPY --from=frontend-build /app/frontend/public/assets ./frontend/public/assets
EXPOSE 8000
CMD ["parking-control"]
