#!/usr/bin/env bash

clang testdns.c -o testdns

echo "# Before hostshook is loaded"
./testdns
echo

echo "# Hostshook is loaded"
export DYLD_INSERT_LIBRARIES=$(pwd)/../../target/debug/libhostshook.dylib
./testdns
echo

echo "# Hostshook is loaded with environment variable"
export HOSTS_ENV=production
./testdns