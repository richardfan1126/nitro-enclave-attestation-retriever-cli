#!/bin/bash

set -e

if [[ -z "${ARCH}" ]]; then
    echo "Please set environment variable ARCH" >&2
    exit 1
fi

docker build . -t nsm-cli --build-arg ARCH=$ARCH

CONTAINER_ID=$(docker create --rm nsm-cli bash)

docker cp $CONTAINER_ID:/root/nsm-cli/target/${ARCH}-unknown-linux-musl/release/nsm-cli ./nsm-cli
docker rm $CONTAINER_ID

strip -s nsm-cli
chmod +x nsm-cli
