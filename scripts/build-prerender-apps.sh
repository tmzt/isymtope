#!/bin/bash
set -e
<<<<<<< HEAD
readonly ROOT=$(realpath $( dirname "${BASH_SOURCE[0]}")/..)
=======
readonly ROOT=${TRAVIS_BUILD_DIR:-"$(realpath $( dirname "${BASH_SOURCE[0]}")/..)"}
>>>>>>> development
readonly APPS_BASE=${WORKER_APP_BASE_URL:-http://localhost:3000/app/playground/_worker/app}

function log() {
    echo $1 > /dev/stderr
}

function prerender() {
    local APP_NAME=$1
    log "Building ${APP_NAME} prerender with base_url (${APPS_BASE})..."
    APP_DIR=./res/tests/app DEFAULT_APP=$APP_NAME ../target/x86_64-unknown-linux-musl/debug/isymtope-cli --base-url $APPS_BASE/todomvc --output ./res/tests/app/${APP_NAME}/index.html /app.ism 2>/dev/null
}

pushd ${ROOT}/isymtope-server
    log "Building isymtope-cli..."
    (cd ../isymtope-cli && cargo build --target x86_64-unknown-linux-musl) && \
        prerender playground && \
        prerender todomvc && \
        prerender shopping
popd
