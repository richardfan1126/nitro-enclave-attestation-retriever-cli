#!/bin/bash

set -e

rm nsm-cli-demo.eif
docker rmi -f nsm-cli-demo:latest
docker build ../.. -f Dockerfile --build-arg ARCH=$(uname -m) -t nsm-cli-demo:latest

nitro-cli build-enclave --docker-uri nsm-cli-demo:latest --output-file nsm-cli-demo.eif

## Uncomment the following lines to run the enclave in debug mode
#nitro-cli run-enclave --cpu-count 2 --memory 2048 --eif-path nsm-cli-demo.eif --debug-mode
#nitro-cli console --enclave-id $(nitro-cli describe-enclaves | jq -r ".[0].EnclaveID")
