# `summertail` Summarize your `tail -f` output, with regexes

> You won't be able to count how many apache 200's are in the logs, so use a regex counter.

## Usage

`summertail DELAY REGEX1 REGEX2 ...`

Reads input from stdin, and for each line keeps track of how many of each REGEX is matched. Every `DELAY` seconds, the counters are reset, and a `\n` is printed.

The last column (`other`) is incremented for every line which doesn't match any of the regexes. If a line matches more than one regex, it will be counted for each regex.

If the `REGEX` starts with `!` it will invert the regex. i.e. it counts lines which _don't_ match that regex.

## Installation

Install with `cargo install summertail`.

# Copyright

Copyright © 2022, GNU Affero GPL licence v3 or later.
Source code: https://github.com/amandasaurus/summertail
