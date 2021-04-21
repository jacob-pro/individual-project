#!/bin/bash
# Compiles release version and copies into /usr/local/bin/
# You can then run kvm-compose to use

set -e
cargo build --release
sudo cp ../target/release/kvm-compose /usr/local/bin/kvm-compose
sudo chown root /usr/local/bin/kvm-compose


