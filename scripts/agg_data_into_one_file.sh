#!/bin/bash
# Get '5-letters' words into file
# Note: You must install ripgrep to use this command
rg -Iz "^[a-zA-Z]{5}_[A-Z]+\t" \
    data/raw/1-*-of-00024.gz \
    > data/parsed/5-letters.txt