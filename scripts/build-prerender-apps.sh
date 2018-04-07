#!/bin/bash
set -e
readonly ROOT=${TRAVIS_BUILD_DIR:-"$(realpath $( dirname "${BASH_SOURCE[0]}")/..)"}
readonly APPS_BASE=${WORKER_APP_BASE_URL:-http://localhost:3000/app/playground/_worker/app}
readonly CONFIGURATION=${BUILD_CONFIGURATION:-release}

readonly CLI_BIN="${ROOT}/target/x86_64-unknown-linux-musl/${CONFIGURATION}/isymtope-cli"

function log() {
    echo $1 > /dev/stderr
}

function prerender() {
    local APP_NAME=$1
    local out="./res/tests/app/${APP_NAME}/index.html"

    log "Building ${APP_NAME} prerender with base_url (${APPS_BASE})..."
    APP_DIR=./res/tests/app ${CLI_BIN} --base-url "${APPS_BASE}/${APP_NAME}/" --output ${out} /app.ism 2>/dev/null
}

pushd ${ROOT}/isymtope-server
    # echo "Checking for existing ${CLI_BIN}..."
    # if [ ! -f $CLI_BIN ]; then
    #     log "Building binaries before prerendered apps..."
    #     ${ROOT}/scripts/build-binaries.sh
    # fi

    prerender playground && \
    prerender todomvc && \
    prerender shopping && \
    prerender materializecss
popd
