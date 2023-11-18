# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2023-11-18

### Added

- DescribePCR subcommand to get PCR value by its index
- Binary for Arm64
- This CHANGELOG file

### Changed

- Upgrade clap version to 4.4.8
- Upgrade aws-nitro-enclaves-nsm-api version to 0.4.0
- Check whether the CLI is running inside Nitro Enclaves before processing request

## [0.1.0] - 2022-02-24

### Added

- Attestation subcommand with "public key", "user data" and "nonce" parameters
- GetRandom subcommand with "length" parameter
- Example Python code to showcase the use of the CLI tool

[unreleased]: https://github.com/richardfan1126/nitro-enclaves-nsm-cli/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/richardfan1126/nitro-enclaves-nsm-cli/releases/tag/v0.2.0
[0.1.0]: https://github.com/richardfan1126/nitro-enclaves-nsm-cli/releases/tag/v0.1.0
