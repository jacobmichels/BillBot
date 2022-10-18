FROM rust:buster AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /app/target/release/billbot /app/billbot

RUN adduser billbot
USER billbot:billbot

ENTRYPOINT ["/app/billbot"]