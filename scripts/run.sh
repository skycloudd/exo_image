#/bin/sh

set -e

input=$1
output=${input%.*}.json

cargo run -- $input $output $2

./scripts/json_to_exolvl.sh $output
