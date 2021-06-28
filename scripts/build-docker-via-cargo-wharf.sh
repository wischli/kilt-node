#!/usr/bin/env bash
cd ../
export DOCKER_BUILDKIT=1

docker build -f Cargo.toml --build-arg debug=all --build-arg profile=release .
