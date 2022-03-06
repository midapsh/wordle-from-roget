#!/bin/bash
awk -F'\t' '{print $1"\t"$NF}' data/parsed/5-letters.txt \
    | sed 's/_/,/' \
    | awk -F, '{print $1" "$(NF-1)}' \
    > data/parsed/5-letters-occur.txt