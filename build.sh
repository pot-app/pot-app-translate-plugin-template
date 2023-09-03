#!/bin/bash

cargo build --target "$INPUT_TARGET" --release
mv target/"$INPUT_TARGET"/release/libplugin.so target/"$INPUT_TARGET"/release/plugin.so
zip $(cat info.json | jq '.id' | tr -d '"').zip info.json icon.svg target/"$INPUT_TARGET"/release/plugin.so