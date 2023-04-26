#!/bin/bash

# Install additional tools

set -e

cargo install cargo-watch
cargo install --version 1.5.0 typeshare-cli
npm install -g @useoptic/optic
cargo install diesel_cli --no-default-features --features postgres
