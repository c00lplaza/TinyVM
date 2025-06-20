FROM rust:1.77 as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/tinyvm /usr/local/bin/tinyvm

CMD ["tinyvm"]