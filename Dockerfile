# build binary
FROM rust:alpine3.18 as builder
RUN apk update

RUN set -x && \
    apk add --no-cache musl-dev openssl-dev openssl-libs-static ca-certificates

# statically link against openssl
ENV OPENSSL_STATIC=1

#build app
COPY ./ /app/
WORKDIR ./app

# Install Clippy and RUn
RUN rustup component add clippy
RUN cargo clippy

ARG SHA=000
RUN VERSION=$SHA cargo build -p sync-rs -r --target=x86_64-unknown-linux-musl

# build final image
FROM amd64/alpine:latest

RUN apk update && apk upgrade
RUN apk add bash ca-certificates openssl
RUN apk add --no-cache tzdata

WORKDIR /opt
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/spartan-rs .
ENV RUST_LOG=info
CMD ["/opt/spartan-rs"]