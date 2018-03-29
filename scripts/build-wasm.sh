#!/bin/bash
set -e
readonly ROOT=${TRAVIS_BUILD_DIR:-"$(realpath $( dirname "${BASH_SOURCE[0]}")/..)"}

function log() {
    echo $1 > /dev/stderr
}

pushd ${ROOT}/wasm-build
    cargo build --verbose --release --target wasm32-unknown-unknown
popd
