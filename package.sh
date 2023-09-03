#!/bin/bash

zip $(cat info.json | jq '.id' | tr -d '"').potext info.json icon.svg target/"$INPUT_TARGET"/release/plugin.$INPUT_EXT