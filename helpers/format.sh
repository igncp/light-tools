#!/usr/bin/env bash

find . \
    ! -path '*node_modules*' \
    ! -path '*.git*' \
    ! -path '*target*' \
    -name '*.rs' | \
  xargs rustfmt "$@"
