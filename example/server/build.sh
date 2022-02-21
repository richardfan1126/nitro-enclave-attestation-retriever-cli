#!/bin/bash

rm nitro-enclave-attestation-retriever-cli.eif
docker rmi -f nitro-enclave-attestation-retriever-cli:latest
docker build ../.. -f Dockerfile --build-arg ARCH=$(uname -m) -t nitro-enclave-attestation-retriever-cli:latest

nitro-cli build-enclave --docker-uri nitro-enclave-attestation-retriever-cli:latest --output-file nitro-enclave-attestation-retriever-cli.eif

## Uncomment the following lines to run the enclave in debug mode
#nitro-cli run-enclave --cpu-count 2 --memory 2048 --eif-path nitro-enclave-attestation-retriever-cli.eif --debug-mode
#nitro-cli console --enclave-id $(nitro-cli describe-enclaves | jq -r ".[0].EnclaveID")
