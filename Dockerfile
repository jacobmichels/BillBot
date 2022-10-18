FROM rust:buster@sha256:129ed2e26c101fc7d03cbb1ef53292e6633ccd6cd147d1c91c2a1df5966a3ee0 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:buster-slim@sha256:89591ac804419bc1f6488c17031b26c7f79062678e3e6fb3e9ae847c5b74b004

COPY --from=builder /app/target/release/billbot /app/billbot

RUN adduser billbot
USER billbot:billbot

ENTRYPOINT ["/app/billbot"]