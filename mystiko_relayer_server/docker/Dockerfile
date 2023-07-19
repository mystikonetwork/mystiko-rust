FROM rust:1.69-buster As builder

COPY . /app/mystiko-rust
WORKDIR /app/mystiko-rust/mystiko_relayer_server
RUN cargo build --release

FROM debian:buster-slim

WORKDIR /app
COPY --from=builder /app/mystiko-rust/mystiko_relayer_server/target/release/mystiko_relayer_server /app
EXPOSE 8090
CMD ["./mystiko_relayer_server", "config.toml"]