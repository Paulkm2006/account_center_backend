FROM rust:alpine as builder
RUN apk update && apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl \
    pkgconfig \
    build-base

WORKDIR /usr/src/account_center_backend
COPY . .

RUN RUSTFLAGS="-C target-feature=-crt-static" cargo install --path . --root /usr/local --target x86_64-unknown-linux-musl

FROM alpine:latest
RUN apk add --no-cache \
    openssl \
    libgcc \
    tzdata && \
    cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime && \
    echo "Asia/Shanghai" > /etc/timezone && \
    apk del tzdata

COPY --from=builder /usr/local/bin/account_center_backend /account_center_backend
CMD ["/account_center_backend"]