#!/bin/bash
# Compiles debug version
# and then immediately runs with script args

set -e
cargo build
sudo cp ../target/debug/kvm-compose kvm-compose
sudo chown root kvm-compose
./kvm-compose "$@"
