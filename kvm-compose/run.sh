#!/bin/bash

set -e
cargo build
sudo cp ../target/debug/kvm-compose kvm-compose
sudo chown root kvm-compose
./kvm-compose "$@"
