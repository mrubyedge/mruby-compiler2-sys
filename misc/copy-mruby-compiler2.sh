#!/bin/bash

SRC_PATH="$1"

if [ -z "$SRC_PATH" ]; then
  echo "Usage: $0 <path-to-mruby-compiler2>"
  exit 1
fi

cp -rv "$SRC_PATH" vendor/