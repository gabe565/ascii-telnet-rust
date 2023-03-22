FROM rust:1.67 as build
WORKDIR /app

# Fix multiplatform build memory issues
# https://github.com/docker/build-push-action/issues/621#issuecomment-1383624173
ARG CARGO_NET_GIT_FETCH_WITH_CLI=true

COPY Cargo.* .
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
