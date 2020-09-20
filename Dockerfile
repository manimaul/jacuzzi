FROM rust:1.46.0-slim-buster as build
## Stage 1

RUN mkdir build_dir
WORKDIR /build_dir

COPY ./Cargo.lock /build_dir/Cargo.lock
COPY ./Cargo.toml /build_dir/Cargo.toml
COPY ./src /build_dir/src

RUN cargo build --release

## Stage 2
FROM debian:buster-slim

COPY --from=build /build_dir/target/release/jacuzzi /usr/bin/jacuzzi
COPY sample /var/www/sample

ENTRYPOINT ["jacuzzi"]
CMD ["/var/www/sample"]
