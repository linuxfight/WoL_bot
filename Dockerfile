FROM rust:alpine as builder

WORKDIR /app

RUN apk add --no-cache openssl-dev musl-dev

COPY . .
RUN RUSTFLAGS='-C target-feature=-crt-static' cargo build --release

FROM alpine
WORKDIR /app

COPY --from=builder /app/target/release .
RUN apk add --no-cache libgcc libstdc++ openssl

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
