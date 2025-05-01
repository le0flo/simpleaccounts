FROM rust:bookworm as builder
WORKDIR /usr/src/simpleaccounts
COPY . .
RUN cargo install --path .

FROM debian:bookworm
COPY --from=builder /usr/local/cargo/bin/simpleaccounts /usr/local/bin/simpleaccounts
ENTRYPOINT [ "simpleaccounts" ]
