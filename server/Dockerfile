FROM alpine:latest

ENV ROCKET_ADDRESS=0.0.0.0

ENV ROCKET_PORT=6969

WORKDIR /app

COPY target/x86_64-unknown-linux-musl/release/server /.

EXPOSE 6969

CMD ["/server"]