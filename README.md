# side-boat 

![build](https://github.com/Netting-mesh/side-boat/workflows/Rust/badge.svg?branch=master) 

Netting is an unopinionated, simple, fast service mesh written in Rust. Its main goal is to solve the
issue service mesh's face now with forcing users into using specific infrastructure as well as being
hard to integrate into an already exisiting kubernetes cluster. Netting will be a plug & play type of 
mesh.

This repository is the sidecar proxy that will be injected into each pod and manage all traffic to the containers within that pod.
[netting](https://github.com/Netting-mesh/netting) is the control plane service that will manage all of the sidecar proxies.

## Installation

### Compiling from source

You can compile `side-boat` by running `cargo build`. You must have Rust Nightly enabled by default to compile.

