This [puzzle](https://adventofcode.com/2021/day/3) requires parsing a list of binary numbers (12 bits each) and determining the most common bit value in each position. The [input](https://adventofcode.com/2021/day/3/input) consists of 1000 entries.

## Part 1
Is it easier to manipulate and query the binary values as strings or by converting them to numeric values? Working with strings means a comparison for character value of '0' or '1'. Working with numeric values means bitwise operations and shifting.

Treating the values as numbers seems more interesting, so I'll go with that option.