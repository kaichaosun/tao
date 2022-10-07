# TAO - Transparent Autonomous Organization

**The repo is replaced by [substrate-stencil](https://github.com/kaichaosun/substrate-stencil) for testnet launch course.**

Using Substrate to build a blockchain which can land the organisations in reality. 

Possible features:

- [ ] create an organisation
- [ ] issue tokens for founders
- [ ] create role for staff
- [ ] define workflow between roles
- [ ] asign tasks under certain role
- [ ] periodcally pay salary for staff
- [ ] deposit initial fund
- [ ] process fund raising
- [ ] create vote for above activity changes

## Components

A new FRAME-based Substrate node, ready for hacking.

## Build

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Initialize your Wasm Build environment:

```bash
./scripts/init.sh
```

Build Wasm and native code:

```bash
cargo build --release
```

## Run

### Single Node Development Chain

Purge any existing developer chain state:

```bash
./target/release/node-template purge-chain --dev
```

Start a development chain with:

```bash
./target/release/node-template --dev
```

Detailed logs may be shown by running the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev`.

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action locally, then you can create a local testnet with two validator nodes for Alice and Bob, who are the initial authorities of the genesis chain that have been endowed with testnet units.

Optionally, give each node a name and expose them so they are listed on the Polkadot [telemetry site](https://telemetry.polkadot.io/#/Local%20Testnet).

You'll need two terminal windows open.

We'll start Alice's substrate node first on default TCP port 30333 with her chain database stored locally at `/tmp/alice`. The bootnode ID of her node is `QmRpheLN4JWdAnY7HGJfWFNbfkQCb6tFf4vvA6hgjMZKrR`, which is generated from the `--node-key` value that we specify below:

```bash
cargo run -- \
  --base-path /tmp/alice \
  --chain=local \
  --alice \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --telemetry-url 'ws://telemetry.polkadot.io:1024 0' \
  --validator
```

In the second terminal, we'll start Bob's substrate node on a different TCP port of 30334, and with his chain database stored locally at `/tmp/bob`. We'll specify a value for the `--bootnodes` option that will connect his node to Alice's bootnode ID on TCP port 30333:

```bash
cargo run -- \
  --base-path /tmp/bob \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/QmRpheLN4JWdAnY7HGJfWFNbfkQCb6tFf4vvA6hgjMZKrR \
  --chain=local \
  --bob \
  --port 30334 \
  --telemetry-url 'ws://telemetry.polkadot.io:1024 0' \
  --validator
```

Additional CLI usage options are available and may be shown by running `cargo run -- --help`.

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can also replace the default command (`cargo build --release && ./target/release/node-template --dev --ws-external`) by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/node-template --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/node-template purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```

## Advanced: Generate Your Own Substrate Node Template

A substrate node template is always based on a certain version of Substrate. You can inspect it by
opening [Cargo.toml](Cargo.toml) and see the template referred to a specific Substrate commit(
`rev` field), branch, or version.

You can generate your own Substrate node-template based on a particular Substrate
version/commit by running following commands:

```bash
# git clone from the main Substrate repo
git clone https://github.com/paritytech/substrate.git
cd substrate

# Switch to a particular branch or commit of the Substrate repo your node-template based on
git checkout <branch/tag/sha1>

# Run the helper script to generate a node template.
# This script compiles Substrate and takes a while to complete. It takes a relative file path
#   from the current dir. to output the compressed node template.
.maintain/node-template-release.sh ../node-template.tar.gz
```

Noted though you will likely get faster and more thorough support if you stick with the releases
provided in this repository.

## ⚡ Run public testnet

* Modify the genesis config in chain_spec.rs
* Build spec, `./target/release/<your-project-name> build-spec --chain staging > my-staging.json`
* Change original spec to encoded raw spec, `./target/release/<your-project-name> build-spec --chain=my-staging.json --raw > my-staging-raw.json`
* Start your bootnodes, node key can be generate with command `./target/release/substrate key generate-node-key`.
  ```shell
  ./target/release/<your-project-name> \
       --node-key <your-node-key> \
       --base-path /tmp/bootnode1 \
       --chain my-staging-raw.json \
       --name bootnode1
  ```
* Start your initial validators,
  ```shell
  ./target/release/<your-project-name> \
      --base-path  /tmp/validator1 \
      --chain   my-staging-raw.json \
      --bootnodes  /ip4/<your-bootnode-ip>/tcp/30333/p2p/<your-bootnode-peerid> \
	  --port 30336 \
	  --ws-port 9947 \
	  --rpc-port 9936 \
      --name  validator1 \
      --validator 
  ```
* [Insert session keys](https://substrate.dev/docs/en/tutorials/start-a-private-network/customchain#add-keys-to-keystore)
* Attract enough validators from community in waiting
* Call force_new_era in staking pallet with sudo, rotate to PoS validators
* Enable governance, and remove sudo
* Enable transfer and other functions 
