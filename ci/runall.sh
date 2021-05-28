#!/usr/bin/env bash

usage() { echo "Usage: $0 -e <environment>" 1>&2; exit 1; }

set -e

declare env=""


while getopts ":e:" arg; do
	case "${arg}" in
		e)
			env=${OPTARG}
			;;

		esac
done
shift $((OPTIND-1))

if [[ -z "$env" ]]; then

    echo "environment is required"
    usage
    exit 1
fi

set +x

source lib-env.sh

set -e
set -x

./build.sh

./push-image.sh -e "${env}"

./deploy.sh -e "${env}"
