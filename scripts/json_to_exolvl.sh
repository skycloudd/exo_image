#!/bin/sh

set -e

gzip "$1"

mv "$1".gz "${1%.*}.exolvl"
