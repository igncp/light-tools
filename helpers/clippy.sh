#!/usr/bin/env bash

set -e

cargo clippy --all-targets --all-features --quiet -- -D warnings
