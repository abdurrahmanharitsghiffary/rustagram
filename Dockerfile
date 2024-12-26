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


WORKDIR /usr/src/app

COPY ./ .

RUN cargo install sqlx-cli --no-default-features --features postgres

RUN cargo build --release


FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl3 \
    pkg-config \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/rustagram /usr/local/bin/
COPY --from=builder /usr/src/app/migrations ./migrations
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
RUN chown rustagram:rustagram /usr/local/bin/sqlx

USER rustagram:rustagram

EXPOSE 8080

CMD ["rustagram"]