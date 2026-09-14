#!/bin/sh
set -eu

cargo build --release

mkdir -p ~/.local/bin
cp target/release/ai-pick ~/.local/bin/

mkdir -p ~/.config/ai-pick
[ -f ~/.config/ai-pick/config.yaml ] || cp config.example.yaml ~/.config/ai-pick/config.yaml

echo "ai-pick installed to ~/.local/bin/ai-pick"
echo "Config: ~/.config/ai-pick/config.yaml"
