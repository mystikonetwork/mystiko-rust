FROM rust:alpine3.18 as builder

COPY . /home/mystiko_rust
WORKDIR /home/mystiko_rust/mystiko_roller
RUN apk add --no-cache musl-dev pkgconfig perl make
RUN cargo install --locked --path .

FROM alpine:3.18.0
COPY --from=builder /usr/local/cargo/bin/mystiko_roller /usr/local/bin/mystiko_roller
CMD ["mystiko_roller"]