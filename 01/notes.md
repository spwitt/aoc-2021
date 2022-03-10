The goal of the puzzle is to count the number of times a depth reading from the [puzzle input](https://adventofcode.com/2021/day/1/input) increases over the previous value.

Rough outline of work:
1. Read the input from text file
2. Convert to Vec of Strings
3. Convert Vec to numeric values
4. Pairwise comparison of adjacent values to increment a counter
5. Output total count

Design:
* Use a command line argument to provide path to the input file rather than hard-coding it
