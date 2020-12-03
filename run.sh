#!/bin/sh
cat "packages/$1/src/input" | cargo run -p $1