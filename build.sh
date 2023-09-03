#!/bin/bash

cargo build --target "$INPUT_TARGET" --release
mv target/"$INPUT_TARGET"/release/libplugin.so target/"$INPUT_TARGET"/release/plugin.so
