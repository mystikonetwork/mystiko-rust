FROM rust:alpine3.18 as builder

COPY . /home/mystiko_rust
WORKDIR /home/mystiko_rust/mystiko_roller
RUN apk add --no-cache musl-dev pkgconfig openssl-dev perl make
RUN cargo install --features build-openssl --locked --path .

FROM alpine:3.18.0
COPY --from=builder /usr/local/cargo/bin/mystiko_roller /usr/local/bin/mystiko_roller
CMD ["mystiko_roller"]