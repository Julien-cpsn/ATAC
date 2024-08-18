ARG RUST_VERSION
FROM lukemathwalker/cargo-chef:latest-rust-$RUST_VERSION-alpine3.20 AS base
WORKDIR /app

FROM base AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin atac

FROM alpine:3.20
COPY --from=builder /app/target/release/atac /atac
WORKDIR /app
ENTRYPOINT [ "/atac" ]
