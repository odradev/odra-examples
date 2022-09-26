# Odra Examples

Example smart contracts written in [Odra](https://github.com/odradev/odra).

Repository contains following modules:
- `ownable` shows how to implement basic access layer.
- `erc20` implements ERC20 token.
- `owned_token` combines `erc20` and `ownable` into a token with an owner, that can mint tokens.
- `balance_checker` shows how to call another contract knowing only part of its interface.

## Prerequisites

- Rust toolchain installed (see [rustup.rs](https://rustup.rs/))
- wasmstrip tool installed (see [wabt](https://github.com/WebAssembly/wabt))

## Usage

To prepare your environment, run:

```bash
make prepare
```

This will install `cargo-odra` and wasm32-unknown-unknown target. 

To run tests against example contracts, run:

```bash
make test
```

Other available commands:

- `make clippy` - run clippy
- `make check-lints` - checks for lint errrors using `cargo fmt`
- `make lint` - runs `cargo fmt`
- `make clean` - removes all generated files