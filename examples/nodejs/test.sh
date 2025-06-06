#!/usr/bin/env bash

echo "# Before hostshook is loaded"
node testdns.js
echo

echo "# Hostshook is loaded"
export DYLD_INSERT_LIBRARIES=$(pwd)/../../target/debug/libhostshook.dylib
node testdns.js
