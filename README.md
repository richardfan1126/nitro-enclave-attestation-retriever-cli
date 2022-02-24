# nitro-enclaves-nsm-cli

**nitro-enclaves-nsm-cli** is a command line tools which runs inside the [Nitro Enclaves](https://aws.amazon.com/ec2/nitro/nitro-enclaves/) and interact with the Nitro Secuity Module (NSM).


This tool is built on top of [Nitro Secure Module library](https://github.com/aws/aws-nitro-enclaves-nsm-api). It wraps the functions into a standalone CLI tool binary. So we can use it directly through any functions in any programming language that can interact with the console. No need to compile the Nitro Secure Module library into the programming language you choose.

## Usage

*This tool only works inside Nitro Enclaves and since you cannot interact with the enclave directly, we need to use it via another program*

```
nsm-cli 

USAGE:
    nsm-cli <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    attest        Generate attestation document
    get-random    Generate random bytes from NSM
    help          Print this message or the help of the given subcommand(s)
```

### Attestation document generation

```
Generate attestation document

USAGE:
    nsm-cli attest [OPTIONS]

OPTIONS:
    -h, --help
            Print help information

    -n, --nonce <NONCE>
            (Optional) Cryptographic nonce provided by the attestation consumer as a proof of
            authenticity

        --nonce-b64 <NONCE_B64>
            (Optional) Base64-encoded cryptographic nonce provided by the attestation consumer as a
            proof of authenticity

    -p, --public-key <PUBLIC_KEY>
            (Optional) DER format public key the attestation consumer can use to encrypt data with

        --public-key-b64 <PUBLIC_KEY_B64>
            (Optional) Base64-encoded DER format public key the attestation consumer can use to
            encrypt data with

    -u, --user-data <USER_DATA>
            (Optional) Additional signed user data

        --user-data-b64 <USER_DATA_B64>
            (Optional) Base64-encoded additional signed user data
```

Example:

```
$ nsm-cli attest -u "some-user-data" -n "some-nonce" --public-key-b64 "MIIB...DAQAB"

# Base64-encoded attestation document returned
hESh...kAwg==
```

### Generate random bytes

```
Generate random bytes from NSM

USAGE:
    nsm-cli get-random --length <LENGTH>

OPTIONS:
    -h, --help               Print help information
    -l, --length <LENGTH>    Byte length of the random data (Maximum 256 bytes)
```

Example:

```
$ nsm-cli get-random -l 256

# Base64-encoded random bytes returned
GnEe...eZcQ==
```

## How to build

### Prerequisite

Docker

### Build steps

```
# Clone the repository
$ git clone https://github.com/richardfan1126/nitro-enclaves-nsm-cli.git

# Run the build script
$ cd nitro-enclaves-nsm-cli
$ ./build.sh

# (Optional) Discards symbols from compiled binary
$ strip -s nsm-cli
```