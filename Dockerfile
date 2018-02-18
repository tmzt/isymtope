FROM alpine:latest

ADD ./target/release/isymtope-server /app
ADD ./isymtope-server/res/tests/app/playground /res/tests/app/playground
ADD ./isymtope-server/res/tests/app/todomvc /res/tests/app/todomvc

ENV DEFAULT_APP playground
ENV APP_DIR "./res/tests/app"

RUN /app