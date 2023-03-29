#!/bin/sh

set -e

mv "$1" "$1".gz

gzip -d "$1".gz

mv "$1" "${1%.*}.json"
