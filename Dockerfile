FROM rust:bookworm AS builder
WORKDIR /usr/src/simpleaccounts

COPY Cargo.toml .
COPY src ./src
RUN cargo install --path .

FROM debian:bookworm
COPY --from=builder /usr/local/cargo/bin/simpleaccounts /usr/local/bin/simpleaccounts
COPY Settings.toml /Settings.toml
ENTRYPOINT [ "simpleaccounts" ]
