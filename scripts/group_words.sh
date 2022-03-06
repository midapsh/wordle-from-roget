#!/bin/bash
awk 'BEGIN {w="";v=0} {if (!w) w=$1; else if ($1==w) v=v+$2; else { print w" "v; w=$1; v=$2; } } END {print w" "v}' \
    < data/parsed/5-letters-lc-sorted.txt \
    > data/parsed/5-letters-lc-sorted-combined.txt