FROM rust:1.63.0-slim-bullseye as builder

# muslc is required in order to build the rust image.
RUN apt-get update && \
    apt-get -y install ca-certificates cmake musl-tools libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo install --path .


FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/athena /usr/local/bin/athena

EXPOSE 8080

CMD ["/usr/local/bin/athena"]
