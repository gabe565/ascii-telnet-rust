FROM --platform=$BUILDPLATFORM rust:1.54-slim as sources
WORKDIR /app

COPY Cargo.* .
# Empty build for dependency cache
# https://github.com/rust-lang/cargo/issues/2644
RUN set -x \
    && mkdir -p .cargo src \
    && touch src/lib.rs \
    && cargo vendor > .cargo/config

FROM rust:1.54-slim as build
WORKDIR /app

COPY --from=sources /app /app
RUN cargo build --release --offline
COPY . .
RUN cargo build --release --offline

FROM debian:stable-slim
COPY --from=build /app/target/release/ascii-telnet /
STOPSIGNAL SIGKILL
CMD ["/ascii-telnet"]
