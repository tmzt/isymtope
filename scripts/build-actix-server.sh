#!/bin/bash
set -e
shopt -s expand_aliases
readonly ROOT=${TRAVIS_BUILD_DIR:-"$(realpath $( dirname "${BASH_SOURCE[0]}")/..)"}
readonly CONFIGURATION=${BUILD_CONFIGURATION:-debug}

. ${ROOT}/scripts/aliases.bash.sh

muslrust cargo build --verbose --target x86_64-unknown-linux-musl --features playground_api -p isymtope-actix
