FROM rust:latest AS builder

RUN update-ca-certificates

ENV USER=rustagram
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /rustagram

COPY ./ .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl3 \
    pkg-config \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /rustagram

COPY --from=builder /rustagram/target/release/rustagram ./

USER rustagram:rustagram

CMD ["/rustagram/rustagram"]