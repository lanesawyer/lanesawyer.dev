FROM debian:13-slim AS base
COPY --from=ghcr.io/jdx/mise:2026.10.14 /usr/local/bin/mise /usr/local/bin/mise
RUN apt-get update \
    && apt-get -y --no-install-recommends install ca-certificates libatomic1 \
    && rm -rf /var/lib/apt/lists/*
ENV MISE_DATA_DIR="/mise"
ENV MISE_CONFIG_DIR="/mise"
ENV MISE_CACHE_DIR="/mise/cache"
ENV PATH="/mise/shims:$PATH"
WORKDIR /app
COPY mise.toml mise.lock ./
RUN mise trust && mise install --locked

FROM base AS build
# Selects S3 (Tigris) media storage; EmDash bakes this choice in at build time.
ENV MEDIA_STORAGE=s3
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
RUN pnpm install --frozen-lockfile
COPY . .
RUN pnpm build

FROM base AS prod-deps
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
RUN pnpm install --frozen-lockfile --prod

FROM base AS runtime
ENV NODE_ENV=production
COPY --from=prod-deps /app/node_modules ./node_modules
COPY --from=build /app/dist ./dist
COPY package.json ./
EXPOSE 8080
CMD ["node", "./dist/server/entry.mjs"]
