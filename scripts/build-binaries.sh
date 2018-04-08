#!/bin/bash
set -e
readonly ROOT=${TRAVIS_BUILD_DIR:-"$(realpath $( dirname "${BASH_SOURCE[0]}")/..)"}
readonly CONFIGURATION=${BUILD_CONFIGURATION:-release}

function log() {
    echo $1 > /dev/stderr
}

pushd ${ROOT} 2>&1 >/dev/null
    echo "Building isymtope binaries..."
    echo "Build configuration ${CONFIGURATION}"
    if [[ "x$CONFIGURATION" == "xrelease" ]]; then
        cargo build --verbose --release --target x86_64-unknown-linux-musl --features playground_api -p isymtope-server -p isymtope-cli
    else
        cargo build --verbose --target x86_64-unknown-linux-musl --features playground_api -p isymtope-server -p isymtope-cli
    fi
popd
