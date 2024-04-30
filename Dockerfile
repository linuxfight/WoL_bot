FROM rust:alpine as builder

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl
RUN apk add --no-cache openssl-dev musl-dev
ENV OPENSSL_DIR=/usr

COPY . .
RUN cargo build --release

FROM alpine
WORKDIR /app

COPY --from=builder /app/target/release .

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
