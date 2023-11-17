#!/bin/bash

set -e

if [[ -z "${ARCH}" ]]; then
    echo "Please set environment variable ARCH" >&2
    exit 1
fi

if [[ "${ARCH}" = "aarch64" ]]; then
    DOCKER_ARCH=arm64
else
    DOCKER_ARCH=$ARCH
fi

docker build . -t nsm-cli --build-arg ARCH=$ARCH --platform linux/$DOCKER_ARCH

CONTAINER_ID=$(docker create --platform linux/$DOCKER_ARCH --rm nsm-cli bash)

docker cp $CONTAINER_ID:/root/nsm-cli/target/${ARCH}-unknown-linux-musl/release/nsm-cli ./nsm-cli
docker rm $CONTAINER_ID

chmod +x nsm-cli
