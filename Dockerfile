FROM --platform=$BUILDPLATFORM rust:1.67 as dependencies
WORKDIR /app

# Improves dependency download speed
ARG CARGO_NET_GIT_FETCH_WITH_CLI=true

COPY Cargo.* .
RUN mkdir .cargo src \
    && touch src/lib.rs
RUN cargo vendor > .cargo/config.toml


FROM rust:1.67 as build
WORKDIR /app

COPY --from=dependencies /app .

# Empty build for dependency cache
# https://github.com/rust-lang/cargo/issues/2644
RUN set -x \
    && mkdir -p src \
    && touch src/lib.rs \
    && cargo build --release

COPY . .
RUN cargo build --release


FROM debian:stable-slim

COPY --from=build /app/target/release/ascii-telnet /usr/local/bin

ARG USERNAME=ascii-telnet
ARG UID=1000
ARG GID=$UID
RUN groupadd --gid "$GID" "$USERNAME" \
    && useradd --uid "$UID" --gid "$GID" -m "$USERNAME"
USER $UID

CMD ["ascii-telnet"]
