FROM alpine:latest

ADD ./target/x86_64-unknown-linux-musl/release/isymtope-server /app
ADD ./isymtope-server/res/tests/app /res/tests/app

ENV DEFAULT_APP playground
ENV APP_DIR "/res/tests/app"

ENTRYPOINT /app