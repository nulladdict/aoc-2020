#!/bin/sh
cat "packages/$1/src/input" | cargo run --release -p $1