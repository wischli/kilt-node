#!/usr/bin/env bash
set +x

export ECR_REPOSITORY="kilt/prototype-chain"
export CACHE_IMAGE_BUILDER_TAG="latest-develop-builder"
export ECR_IMAGE_TAG="latest-develop"
export SHA_IMAGE_TAG=`git rev-parse origin/develop`
export KUBECONFIG=`readlink -f ~/.kube/config`
export ARTIFACT_DEST_PATH="mashnet_node_runtime.compact.wasm"
