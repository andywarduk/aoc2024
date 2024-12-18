#!/bin/bash

if [ "x$1" == "x" ]; then
	echo "Must give a day number"
	exit 1
fi

if [ "$1" -lt 1 -o "$1" -gt 25 ]; then
	echo "Day number invalid"
	exit 1
fi

daypad="$(printf %02d $1)"

mkdir -p flamegraphs >/dev/null 2>&1

file=flamegraphs/day$daypad-rel.svg

cargo flamegraph --root --profile profile --bin day$daypad --output "$file"

open "$file"
