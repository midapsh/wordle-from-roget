#!/bin/bash
# Modify json data, sort the data by line and put into a file
# Note 1: answers.json´ is the data that you get for NY times Wordle
# puzzle. It's, as already said, the valid words to use in the game
# Note 2: you must install this lib
#  ´sudo apt install jq´
jq -r '.[]' data/answers.json | sort > data/wordle.txt