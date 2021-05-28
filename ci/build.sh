#!/usr/bin/env bash

set -e

printf "\n\nBuilding mashnet-node ...\n\n"


source lib-env.sh

echo "Building ${module} ${VERSION}..."

../scripts/init.sh

cargo build --release -p mashnet-node


