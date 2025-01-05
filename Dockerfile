FROM rust:1.72 AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim


WORKDIR /app
COPY --from=builder /usr/src/app/target/release/<binary_name> /app/<binary_name>

EXPOSE 50051

CMD ["./<binary_name>"]
