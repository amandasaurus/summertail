# `summertail` Summarize your `tail -f` output, with regexes

> You won't be able to count how many apache 200's are in the logs, so use a regex counter.

> Watching hundreds of lines of log files scroll by doesn't mean you know what happened

## Usage

`summertail DELAY REGEX1 REGEX2 ...`

Reads input from stdin, and for each line keeps track of how many of each REGEX is matched. Every `DELAY` seconds, the counters are reset, and a `\n` is printed.

The last column (`other`) is incremented for every line which doesn't match any of the regexes. If a line matches more than one regex, it will be counted for each regex.

If the `REGEX` starts with `!` it will invert the regex. i.e. it counts lines which _don't_ match that regex.

## Installation

Install with `cargo install summertail`.

## Example

```bash
$ tail -f /var/log/apache2/access.log | summertail 10s ' 200 ' ' 3\d\d ' \!' 200 '
🕛︎10:16:22 27 ▃▍   200 : 17 ( 63%) ▆▊   3\d\d : 10 ( 37%) ▃▍  ! 200 : 10 ( 37%) ▃▍  other:  0 (  0%)     
🕛︎10:16:32 37 ▄▌   200 : 15 ( 41%) ▄▌   3\d\d : 19 ( 51%) ▅▋  ! 200 : 22 ( 59%) ▅▋  other:  0 (  0%)     
🕚︎10:16:41 23 ▃▍   200 :  8 ( 35%) ▃▍   3\d\d : 15 ( 65%) ▆▊  ! 200 : 15 ( 65%) ▆▊  other:  0 (  0%)     
🕛︎10:16:52 23 ▃▍   200 :  6 ( 26%) ▃▍   3\d\d : 16 ( 70%) ▆▊  ! 200 : 17 ( 74%) ▆▊  other:  0 (  0%)     
🕛︎10:17:02 47 ▅▋   200 : 11 ( 23%) ▂▎   3\d\d : 34 ( 72%) ▆▊  ! 200 : 36 ( 77%) ▇▉  other:  0 (  0%)     
🕛︎10:17:13 41 ▄▌   200 : 13 ( 32%) ▃▍   3\d\d : 24 ( 59%) ▅▋  ! 200 : 28 ( 68%) ▆▊  other:  0 (  0%)     
🕛︎10:17:23 48 ▅▋   200 : 20 ( 42%) ▄▌   3\d\d : 27 ( 56%) ▅▋  ! 200 : 28 ( 58%) ▅▋  other:  0 (  0%)     
🕛︎10:17:33 53 ▆▊   200 : 25 ( 47%) ▄▌   3\d\d : 28 ( 53%) ▅▋  ! 200 : 28 ( 53%) ▅▋  other:  0 (  0%)     
🕛︎10:17:44 30 ▃▍   200 :  9 ( 30%) ▃▍   3\d\d : 21 ( 70%) ▆▊  ! 200 : 21 ( 70%) ▆▊  other:  0 (  0%)     
🕛︎10:17:54 79 ██   200 : 31 ( 39%) ▄▌   3\d\d : 43 ( 54%) ▅▋  ! 200 : 48 ( 61%) ▅▋  other:  0 (  0%)     
🕛︎10:18:04 129 ██   200 :  69 ( 53%) ▅▋   3\d\d :  60 ( 47%) ▄▌  ! 200 :  60 ( 47%) ▄▌  other:   0 (  0%)     
🕛︎10:18:14  19 ▂▎   200 :   5 ( 26%) ▃▍   3\d\d :  14 ( 74%) ▆▊  ! 200 :  14 ( 74%) ▆▊  other:   0 (  0%)     
🕚︎10:18:24  62 ▅▋   200 :  22 ( 35%) ▃▍   3\d\d :  38 ( 61%) ▅▋  ! 200 :  40 ( 65%) ▆▊  other:   0 (  0%)     
🕚︎10:18:34 159 ██   200 :  20 ( 13%) ▂▎   3\d\d : 138 ( 87%) ▇▉  ! 200 : 139 ( 87%) ▇▉  other:   0 (  0%)     
🕛︎10:18:44  34 ▂▎   200 :   4 ( 12%) ▁▏   3\d\d :  30 ( 88%) ██  ! 200 :  30 ( 88%) ██  other:   0 (  0%)     
🕛︎10:18:55  50 ▃▍   200 :  10 ( 20%) ▂▎   3\d\d :  40 ( 80%) ▇▉  ! 200 :  40 ( 80%) ▇▉  other:   0 (  0%)
```

# Copyright

Copyright © 2022, GNU Affero GPL licence v3 or later.
Source code: https://github.com/amandasaurus/summertail
