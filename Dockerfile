FROM alpine:latest

ADD ./target/x86_64-unknown-linux-musl/release/isymtope-server /app
ADD ./isymtope-server/res/tests/app/playground /res/tests/app/playground
ADD ./isymtope-server/res/tests/app/todomvc /res/tests/app/todomvc

ENV DEFAULT_APP playground
ENV APP_DIR "./res/tests/app"

ENTRYPOINT /app