# The Video behind this
- [Link](https://www.youtube.com/watch?v=doFowk4xj7Q&t=4s)

# The Git behind this
- [Link](https://github.com/jonhoo/roget)

# The Guy behind this
- [Link](https://github.com/jonhoo/roget)

# The README.md behind this
Original version live-coded [on YouTube](https://youtu.be/doFowk4xj7Q).

The implemented algorithm is almost exactly what was outlined (and
_very_ well explained) in [this 3blue1brown video][3b1b].

Please do tinker with it and see how much you can push it — there's
almost certainly gains to be had! I've also left some TODOs from the
3b1b algorithm that should improve the guesses a fair bit. It'd also be
really neat to add in a mode for computing the _first_ word by computing
multiple levels of expected information (again, like 3b1b), instead of
just hard-coding it like we do at the moment.


# Dataset

If you want to remake `dictionary.txt` yourself, first, make
`answers.json` by grabbing the words from the [Wordle source code][ny-times]. Then, grab the ngram dataset by
downloading [these][1grams], then running.

```bash
source scripts/create_folders.sh
source scripts/agg_data_into_one_file.sh
source scripts/get_words_occurrences.sh
source scripts/sort_words_occurrences.sh
source scripts/group_words.sh
source scripts/create_wordle_file.sh
source scripts/get_dictionary.sh
```

[3b1b]: https://www.youtube.com/watch?v=v68zYyaEmEA
[1grams]: https://storage.googleapis.com/books/ngrams/books/20200217/eng/eng-1-ngrams_exports.html
[ny-times]: https://www.nytimes.com/games/wordle/index.html