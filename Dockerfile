FROM rust:slim AS builder

WORKDIR /usr/src/icsdt-lab1-app

RUN apt-get update && apt-get install -y libpq-dev libssl-dev pkg-config

COPY Cargo.toml .
COPY Cargo.lock .
COPY src src

RUN rustup update nightly
RUN cargo +nightly build --release -Z sparse-registry -j16

FROM debian:bullseye-slim

WORKDIR /usr/src/icsdt-lab1-app

RUN apt-get update && apt-get install -y libpq-dev

COPY --from=builder /usr/src/icsdt-lab1-app/target/release/icsdt-lab1-app .
# COPY .env .

EXPOSE 8100

CMD ["./icsdt-lab1-app"]