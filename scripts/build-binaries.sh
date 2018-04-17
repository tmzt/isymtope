#!/bin/bash
set -e
readonly ROOT=${TRAVIS_BUILD_DIR:-"$(realpath $( dirname "${BASH_SOURCE[0]}")/..)"}
readonly CONFIGURATION=${BUILD_CONFIGURATION:-release}

readonly OPENSSL_VERS=${BUILD_OPENSSL_VERS:-1.0.2j}

function log() {
    echo $1 > /dev/stderr
}

function build_openssl() {
    # Download and build OpenSSL against musl
    export CC=musl-gcc
    export MUSL_PREFIX=/usr/local/musl
    export C_INCLUDE_PATH="/usr/include/x86_64-linux-musl:$MUSL_PREFIX/include/"

    [ -f openssl-$OPENSSL_VERS.tar.gz ] || curl -# -O https://www.openssl.org/source/openssl-$OPENSSL_VERS.tar.gz
    [ -f /usr/lib/x86_64-linux-musl/lib/libssl.a ] || (
        tar xzf openssl-$OPENSSL_VERS.tar.gz
        cd openssl-$OPENSSL_VERS

        ./config --prefix="$MUSL_PREFIX"
        # make depend
        make
        sudo make install
    )
}

export OPENSSL_DIR=/usr/lib/x86_64-linux-musl
export OPENSSL_STATIC=1

readonly FEATURES="playground_api,redis_session,github_auth,site_app"

# required for openssl-sys
export PKG_CONFIG_ALLOW_CROSS=1

pushd ${ROOT} 2>&1 >/dev/null
    echo "Building openssl..."
    build_openssl

    echo "Building isymtope binaries..."
    echo "Build configuration ${CONFIGURATION}"
    if [[ "x$CONFIGURATION" == "xrelease" ]]; then
        cargo build --verbose --release --target x86_64-unknown-linux-musl --features ${FEATURES} -p isymtope-server -p isymtope-cli
    else
        cargo build --verbose --target x86_64-unknown-linux-musl --features ${FEATURES} -p isymtope-server -p isymtope-cli
    fi
popd
