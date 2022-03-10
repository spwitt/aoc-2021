The goal of the puzzle is to count the number of times a depth reading from the [puzzle input](https://adventofcode.com/2021/day/1/input) increases over the previous value.

In the second part of the puzzle the sums for overlapping groups of 3 elements were calculated and then counted the number of increases between elements of those sums.

Rough outline of work:
1. Read the input from text file
2. Convert to Vec of Strings
3. Convert Vec to numeric values
4. Pairwise comparison of adjacent values to increment a counter
5. Output total count

Design:
* Use a command line argument to provide path to the input file rather than hard-coding it

Commentary/Notes:
* The goal here was to come to a solution quickly and get back into writing Rust code
* The code does not have any error handling
* Too much of the code is in the `main` function
* No testing. It would be good to test both the input parsing _and_ the processing
* The [`windows`](https://doc.rust-lang.org/std/slice/struct.Windows.html) method ended up being _very_ useful here
  * It provided an easy way to grab two elements at a time for comparison on part 1
  * Automatically handled/prevented not stepping off the end of the array
  * Very easy to reuse for calculating the sums for sliding windows in part 2