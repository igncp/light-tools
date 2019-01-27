#!/usr/bin/env bash

set -e

sh ../../helpers/format.sh

sh ../../helpers/clippy.sh

echo "All correct"
