#!/bin/bash
tr A-Z a-z \
    < data/parsed/5-letters-occur.txt \
    | sort \
    > data/parsed/5-letters-lc-sorted.txt