#!/bin/bash

set -e

ARCH=$(uname -m)

docker build . -t nsm-cli --build-arg ARCH=$ARCH

CONTAINER_ID=$(docker create --rm nsm-cli bash)

docker cp $CONTAINER_ID:/root/nsm-cli/target/${ARCH}-unknown-linux-musl/release/nsm-cli ./nsm-cli
docker rm $CONTAINER_ID

chmod +x nsm-cli