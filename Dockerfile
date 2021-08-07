FROM rust:1.54-slim as build
WORKDIR /app

COPY Cargo.* .
# Empty build for dependency cache
# https://github.com/rust-lang/cargo/issues/2644
RUN set -x \
    && mkdir src \
    && touch src/lib.rs \
    && cargo build --release

COPY . .
RUN cargo build --release

FROM debian:stable-slim
COPY --from=build /app/target/release/ascii-telnet /
STOPSIGNAL SIGKILL
CMD ["/ascii-telnet"]
