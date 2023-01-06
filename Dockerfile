FROM rust:1.66 as build
WORKDIR /app

COPY Cargo.* .
# Empty build for dependency cache
# https://github.com/rust-lang/cargo/issues/2644
RUN set -x \
    && mkdir -p src \
    && touch src/lib.rs \
    && cargo build --release

COPY . .
RUN cargo build --release

FROM debian:buster-slim
LABEL org.opencontainers.image.authors="Gabe Cook <gabe565@gmail.com>"
LABEL org.opencontainers.image.source="https://github.com/gabe565/ascii-telnet-rust"

COPY --from=build /app/target/release/ascii-telnet /usr/local/bin

ARG USERNAME=ascii-telnet
ARG UID=1000
ARG GID=$UID
RUN groupadd --gid "$GID" "$USERNAME" \
    && useradd --uid "$UID" --gid "$GID" -m "$USERNAME"
USER $UID

CMD ["ascii-telnet"]
