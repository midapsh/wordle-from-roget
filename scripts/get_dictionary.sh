#!/bin/bash
# Get legal words by doing a inner join
# Note: you must get the legal words and put into wordle.txt file.
# Then, you should run the command to get the dictionary.txt
join -a 1 \
    data/wordle.txt \
    data/parsed/5-letters-lc-sorted-combined.txt \
    | sed 's/\([a-z]\)$/\1 1/' \
    > data/dictionary.txt