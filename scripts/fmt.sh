#!/bin/bash
set -e

if ! cargo +nightly --version &> /dev/null; then
    exit 1
fi

cd ..

cargo +nightly fmt --all
