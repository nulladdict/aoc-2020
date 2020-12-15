#!/bin/sh
cargo run -p $1  ${@:2} < "packages/$1/src/input"