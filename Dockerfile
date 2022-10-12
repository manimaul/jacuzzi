FROM gcr.io/distroless/base

COPY ./target/x86_64-unknown-linux-musl/release/jacuzzi /bin/jacuzzi
COPY sample /var/www/sample
COPY self_signed_cert /etc/jacuzzi/tls
ENV TLS_DIR /etc/jacuzzi/tls

ENTRYPOINT ["jacuzzi"]
CMD ["/var/www/sample"]
