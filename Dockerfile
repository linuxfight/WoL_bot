FROM rust:slim-bookworm as builder

WORKDIR /app

RUN apt update -y && apt install openssl libssl-dev pkg-config -y

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/target/release .
RUN apt update -y && apt install openssl -y

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

CMD ["/app/wtp"]
