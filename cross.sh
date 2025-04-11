#!/bin/bash

set -x

TARGET=armv7-unknown-linux-musleabihf
INSTALL_DIR=install/usr

mkdir -p install/usr
sh -c "cross-util run --target $TARGET -- \"$* --locked --root $INSTALL_DIR --target $TARGET\""
