#!/bin/bash
rustup component add clippy rustfmt
cargo fetch
#rustup target add wasm32-unknown-unknown

chmod 600 /root/.ssh/id_* && chmod 700 /root/.ssh
echo "Setup complete!"

