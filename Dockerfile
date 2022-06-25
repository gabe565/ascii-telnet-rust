ARG VERSION=1.61

FROM --platform=$BUILDPLATFORM rust:$VERSION-alpine as build
WORKDIR /app

RUN apk add \
        musl-dev

ARG CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=/usr/bin/arm-none-eabi-gcc
ARG CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=/usr/bin/aarch64-none-elf-gcc

# Convert Docker platform string to Rust target string
ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
      'linux/amd64') \
          echo x86_64-unknown-linux-musl >/rust_target \
          ;; \
      'linux/arm/v7') \
          echo armv7-unknown-linux-musleabihf >/rust_target \
          && apk add gcc-arm-none-eabi \
          ;; \
      'linux/arm64') \
          echo aarch64-unknown-linux-musl >/rust_target \
          && apk add gcc-aarch64-none-elf \
          ;; \
      *) echo "Unsupported target: $TARGETPLATFORM" && exit 1 ;; \
    esac \
    && rustup target add "$(cat /rust_target)"

COPY Cargo.* .
# Empty build for dependency cache
# https://github.com/rust-lang/cargo/issues/2644
RUN set -x \
    && mkdir -p src \
    && touch src/lib.rs \
    && cargo build --release --target "$(cat /rust_target)"

COPY . .
RUN cargo build --release --target "$(cat /rust_target)"

FROM alpine
LABEL org.opencontainers.image.authors="Gabe Cook <gabe565@gmail.com>"
LABEL org.opencontainers.image.source="https://github.com/gabe565/ascii-telnet-rust"

COPY --from=build /app/target/*/release/ascii-telnet /usr/local/bin
STOPSIGNAL SIGKILL
CMD ["ascii-telnet"]
