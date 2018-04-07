#!/bin/bash
set -e
readonly ROOT=${TRAVIS_BUILD_DIR:-"$(realpath $( dirname "${BASH_SOURCE[0]}")/..)"}

${ROOT}/scripts/build-binaries.sh
${ROOT}/scripts/build-prerender-apps.sh
