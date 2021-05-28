#!/usr/bin/env bash

usage() { echo "Usage: $0  -e <environment> " 1>&2; exit 1; }

set -e

declare env=""


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

set +x
set -e

source lib-env.sh

mkdir -p '~/.kube' \
          && echo '${{ secrets.KUBE_CONFIG}}' | base64 -d > $KUBECONFIG

if [[ "${env}" == "dev" ]]; then
 NAMESPACE="devnet"
else
 NAMESPACE="mashnet"
fi

kubectl config use-context arn:aws:eks:eu-central-1:348099934012:cluster/${env}
kubectl -n ${NAMESPACE} set image deployment/node-alice-deployment node-container=$ECR_REGISTRY/$ECR_REPOSITORY:$SHA_IMAGE_TAG
kubectl -n ${NAMESPACE} set image deployment/node-bob-deployment node-container=$ECR_REGISTRY/$ECR_REPOSITORY:$SHA_IMAGE_TAG
kubectl -n ${NAMESPACE} set image deployment/node-charlie-deployment node-container=$ECR_REGISTRY/$ECR_REPOSITORY:$SHA_IMAGE_TAG
kubectl -n ${NAMESPACE} set image statefulset/node-full-deployment node-container=$ECR_REGISTRY/$ECR_REPOSITORY:$SHA_IMAGE_TAG
