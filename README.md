Rust-Lightning-Wizards
======================

[![Crate](https://img.shields.io/crates/v/lightning.svg?logo=rust)](https://crates.io/crates/lightning)
[![Documentation](https://img.shields.io/static/v1?logo=read-the-docs&label=docs.rs&message=lightning&color=informational)](https://docs.rs/lightning/)
[![Safety Dance](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

A fork of rust-lightning with support for advanced and experimental features such as custom scripts
and advanced LSP tooling.

The primary crate, `lightning`, is runtime-agnostic. Data persistence, chain interactions,
and networking can be provided by LDK's [sample modules](#crates), or you may provide your
own custom implementations.
More information is available in the [`About`](#about) section.

Status
------
The project implements all of the [BOLT specifications](https://github.com/lightning/bolts),
and has been in production use since 2021. As with any Lightning implementation, care and attention
to detail is important for safe deployment.

Crates
-----------
1. [lightning](./lightning)
  The core of the LDK library, implements the Lightning protocol, channel state machine,
  and on-chain logic. Supports `no-std` and exposes only relatively low-level interfaces.
2. [lightning-background-processor](./lightning-background-processor)
  Utilities to perform required background tasks for Rust Lightning.
3. [lightning-block-sync](./lightning-block-sync)
  Utilities to fetch the chain data from a block source and feed them into Rust Lightning.
4. [lightning-invoice](./lightning-invoice)
  Data structures to parse and serialize
  [BOLT #11](https://github.com/lightning/bolts/blob/master/11-payment-encoding.md)
  Lightning invoices.
5. [lightning-net-tokio](./lightning-net-tokio)
  Implementation of the `rust-lightning` network stack using the
  [Tokio](https://github.com/tokio-rs/tokio) `async` runtime. For `rust-lightning`
  clients which wish to make direct connections to Lightning P2P nodes, this is
  a simple alternative to implementing the required network stack, especially
  for those already using Tokio.
6. [lightning-persister](./lightning-persister)
  Implements utilities to manage `rust-lightning` channel data persistence and retrieval.
  Persisting channel data is crucial to avoiding loss of channel funds.
7. [lightning-rapid-gossip-sync](./lightning-rapid-gossip-sync)
  Client for rapid gossip graph syncing, aimed primarily at mobile clients.

Design Goal
-----------
The goal is to provide a fully-featured and incredibly flexible Lightning
implementation, allowing users to decide how they wish to use it. With that
in mind, everything should be exposed via simple, composable APIs. More
information about `rust-lightning`'s flexibility is provided in the `About`
section above.

For security reasons, do not add new dependencies. Really do not add new
non-optional/non-test/non-library dependencies. Really really do not add
dependencies with dependencies. Do convince Andrew to cut down dependency usage
in `rust-bitcoin`.

Rust-Lightning vs. LDK (Lightning Development Kit)
-------------
`rust-lightning` refers to the core `lightning` crate within this repo, whereas
LDK encompasses `rust-lightning` and all of its sample modules and crates (e.g.
the `lightning-persister` crate), language bindings, sample node
implementation(s), and other tools built around using `rust-lightning` for
Lightning integration or building a Lightning node.

Tagline
-------

*"Rust-Lightning, not Rusty's Lightning!"*

Contributing
------------

Contributors are warmly welcome, see [CONTRIBUTING.md](CONTRIBUTING.md).

Project Architecture
---------------------

For a `rust-lightning` high-level API introduction, see [ARCH.md](ARCH.md).

License is either Apache-2.0 or MIT, at the option of the user (ie dual-license
Apache-2.0 and MIT).
