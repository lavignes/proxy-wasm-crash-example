#!/bin/bash

cleanup() {
  rm -f proxy_wasm_crash_example.wasm
  popd > /dev/null || 0
}

build_and_run() {
  docker build -f Dockerfile -t proxy-wasm-crash-example . || exit
  docker run -p 8080:8080 --rm -it proxy-wasm-crash-example || exit
}

pushd "$(dirname "$0")" > /dev/null || exit
trap cleanup EXIT

cp target/wasm32-unknown-unknown/debug/proxy_wasm_crash_example.wasm ./

build_and_run "$1"
