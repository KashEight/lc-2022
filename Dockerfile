FROM rust:1-slim-bullseye AS builder

WORKDIR /app

RUN cargo init
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src src

RUN touch src/main.rs

RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=builder /app/target/release/lc-2022 /
COPY static static

EXPOSE 8080

CMD ["./lc-2022"]