# Ekiden token contract

[![CircleCI](https://circleci.com/gh/ekiden/contract-token/tree/master.svg?style=svg)](https://circleci.com/gh/ekiden/contract-token/tree/master)

## Checking out

The repository uses submodules so be sure to check them out by doing:
```bash
$ git submodule update --init --recursive
```

## Building the contract

The easiest way to build SGX code is to use the provided scripts, which run a Docker
container with all the included tools. This has been tested on MacOS and Ubuntu with `SGX_MODE=SIM`.

To start the SGX development container:
```bash
$ ./ekiden/scripts/sgx-enter.sh
```

Ekiden uses [`cargo-make`](https://crates.io/crates/cargo-make) as the build system. The
development Docker container already comes with `cargo-make` preinstalled.

First, build Ekiden to get the correct core contracts. This is currently needed because
the core contracts are not yet finalized and it is important that correct hashes are used.
You can do that by running:
```bash
$ cd ekiden
$ cargo make
$ cd ..
```

After the process completes, to build everything required for running Ekiden, simply run
the following in the top-level directory:
```bash
$ cargo make
```

For subsequent rebuilds, you only need to perform this step while rebuilding Ekiden is not
required.

This should install any required dependencies and build all packages. By default SGX code is
built in simulation mode. To change this, do `export SGX_MODE=HW` (currently untested) before
running the `cargo make` command.

The built contract will be stored under `target/enclave/token.signed.so`.

## Building the example client

The example client is located under `examples/client` and it may be built using:
```bash
$ cd examples/client
$ cargo build
```

## Running the contract

For running the built contract consult the Ekiden documentation.
