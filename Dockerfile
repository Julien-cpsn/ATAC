ARG RUST_VERSION
FROM --platform=$BUILDPLATFORM lukemathwalker/cargo-chef:latest-rust-$RUST_VERSION-alpine3.20 AS base
WORKDIR /app

FROM base AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base AS builder
ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
	"linux/amd64") echo "x86_64-unknown-linux-musl" > rust_target.txt ;; \
	"linux/arm64") echo "aarch64-unknown-linux-musl" > rust_target.txt ;; \
	esac && \
	# Install musl target
	rustup target add $(cat rust_target.txt) && \
	apk add zig && \
	cargo install cargo-zigbuild

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target $(cat rust_target.txt) --recipe-path recipe.json --zigbuild
COPY . .
RUN cargo zigbuild --release --target $(cat rust_target.txt) --bin atac && cp /app/target/$(cat rust_target.txt)/release/atac /atac

FROM alpine:3.20 AS runtime
COPY --from=builder /atac /atac
WORKDIR /app
ENTRYPOINT [ "/atac" ]
