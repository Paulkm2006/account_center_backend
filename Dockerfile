FROM rust:alpine as builder
WORKDIR /usr/src/account_center_backend
COPY . .
RUN apk update && apk add musl-dev openssl-dev
RUN cargo install --path . --root /usr/local --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /usr/local/bin/account_center_backend /account_center_backend
CMD ["/account_center_backend"]