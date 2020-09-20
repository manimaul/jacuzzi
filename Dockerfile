FROM rust:1.46.0-slim-buster as build
## Stage 1

RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN mkdir build_dir
WORKDIR /build_dir

COPY ./Cargo.lock /build_dir/Cargo.lock
COPY ./Cargo.toml /build_dir/Cargo.toml
COPY ./src /build_dir/src

RUN cargo build --release

## Stage 2
FROM debian:buster-slim

ENV DEBIAN_FRONTEND noninteractive
RUN apt-get update && \
    apt-get -y upgrade &&\
    apt-get -y install --no-install-recommends libssl1.1 && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /build_dir/target/release/jacuzzi /usr/bin/jacuzzi
COPY sample /var/www/sample
COPY self_signed_cert /etc/jacuzzi/tls
ENV TLS_DIR /etc/jacuzzi/tls

ENTRYPOINT ["jacuzzi"]
CMD ["/var/www/sample"]
