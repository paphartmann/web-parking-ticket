FROM docker.io/library/node:22-bookworm-slim AS frontend
WORKDIR /app/frontend
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci
COPY frontend/ ./
RUN npm run build

FROM docker.io/library/rust:1-bookworm AS api
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build
EXPOSE 8000
CMD ["target/debug/parking-control"]

FROM docker.io/library/nginx:1.27-alpine AS nginx
COPY nginx.conf /etc/nginx/nginx.conf
COPY frontend/public/ /var/www/
COPY --from=frontend /app/frontend/public/assets/ /var/www/assets/
EXPOSE 80
