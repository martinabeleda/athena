FROM rust:1.63.0-slim-bullseye as builder

RUN apt-get update && \
    apt-get -y install ca-certificates cmake musl-tools libssl-dev libpq-dev && \
    rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo install --path .


FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/athena /usr/local/bin/athena

RUN apt-get update && \
    apt-get -y install libpq-dev && \
    rm -rf /var/lib/apt/lists/*

EXPOSE 8080

CMD ["/usr/local/bin/athena"]
