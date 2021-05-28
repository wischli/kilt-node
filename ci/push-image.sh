#!/usr/bin/env bash

usage() { echo "Usage: $0  -e <environment> " 1>&2; exit 1; }

set -e

declare env=""
declare module=""


# Initialize parameters specified from command line
while getopts "e:" arg; do
	case "${arg}" in
		e)
			env=${OPTARG}
			;;
		esac
done
shift $((OPTIND-1))


i
if [[ -z "$env" ]]; then

    echo "Deployment environment is required"
    usage
    exit 1
fi


if [[ "${env}" == "dev" ]]; then
 REGISTRY="kilt/prototype-chain"
else
 REGISTRY="kiltprotocol/mashnet-node"
fi


source lib-env.sh
docker build \
          -t $REGISTRY/$ECR_REPOSITORY:$ECR_IMAGE_TAG \
          -t $REGISTRY/$ECR_REPOSITORY:$SHA_IMAGE_TAG \
          --build-arg NODE_TYPE=mashnet-node \
          .
docker push $REGISTRY/$ECR_REPOSITORY:$ECR_IMAGE_TAG
docker push $REGISTRY/$ECR_REPOSITORY:$SHA_IMAGE_TAG
echo "::set-output name=image::$REGISTRY/$ECR_REPOSITORY:$ECR_IMAGE_TAG"
