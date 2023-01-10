# Mystiko.Network - The Layer P(rivacy) of Web 3.0

[![build status](https://github.com/mystikonetwork/mystiko-core/actions/workflows/build.yml/badge.svg)](https://github.com/mystikonetwork/mystiko-core/actions/workflows/build.yml)

[Mystiko.Network](https://mystiko.network) is the base layer Of Web3 via zero knowledge proof based cross-chain networks.
This repository contains the implementation of Mystiko's core protocol in Rust. Please check our
[Whitepaper](https://mystiko.network/whitepaper.pdf) for more formal information about the protocol.

Our zero knowledge proof primitives are built based on [Circom](https://github.com/iden3/circom), which is a popular
zkSnark compiler and toolchain. Here is the details of the proving schemes we are using:
* **alt_bn128** - a pairing-friendly elliptic curve, which is also efficient on EVM.
* **[groth16](https://eprint.iacr.org/2016/260)** - a proving scheme could work with the alt_bn128 curves.
* **[snarkjs](https://github.com/iden3/snarkjs)** - a zkSnark backend and runtime.
