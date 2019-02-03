#!/usr/bin/env bash

set -e

cargo build --release

sudo rm /usr/bin/o

sudo mv target/release/o /usr/bin
