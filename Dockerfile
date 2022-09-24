ARG VERSION=1.63

FROM rust:$VERSION-alpine as build
WORKDIR /app

RUN apk add --no-cache musl-dev

COPY Cargo.* .
# Empty build for dependency cache
# https://github.com/rust-lang/cargo/issues/2644
RUN set -x \
    && mkdir -p src \
    && touch src/lib.rs \
    && cargo build --release

COPY . .
RUN cargo build --release

FROM alpine
LABEL org.opencontainers.image.authors="Gabe Cook <gabe565@gmail.com>"
LABEL org.opencontainers.image.source="https://github.com/gabe565/ascii-telnet-rust"

COPY --from=build /app/target/release/ascii-telnet /usr/local/bin

ARG USERNAME=ascii-telnet
ARG UID=1000
ARG GID=$UID
RUN addgroup -g "$GID" "$USERNAME" \
    && adduser -S -u "$UID" -G "$USERNAME" "$USERNAME"
USER $UID

CMD ["ascii-telnet"]
