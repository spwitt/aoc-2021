This [puzzle](https://adventofcode.com/2021/day/3) requires parsing a list of binary numbers (12 bits each) and determining the most common bit value in each position. The [input](https://adventofcode.com/2021/day/3/input) consists of 1000 entries.

## Part 1
Is it easier to manipulate and query the binary values as strings or by converting them to numeric values? Working with strings means a comparison for character value of '0' or '1'. Working with numeric values means bitwise operations and shifting.

Treating the values as numbers seems more interesting, so I'll go with that option.

## Part 2
To get the required values for the second part, we need to get values for "oxygen generator rating" and "CO2 scrubber rating."

In both cases we start with the most significant bit and find which bit is the most common. Oxygen is filtered to values with the _more common_ value in that position (or where value is `1` in case of a tie) while CO2 is filtered to values with the _less common_ value in that position (or where value is `0` in case of a tie).

The process is repeated until there is just one value left for each rating. This may occur before all bits are processed.

Multiply the two ratings together to get the answer to the puzzle.

Unfortunately my code for the first part does not play so nicely with this second part. It may be easier to change to using the strings here, but I will try to keep using the numeric values. I'll start by refactoring my code to get a _count_ of the values in a vector that match a bitmask. This is kind of an inversion of what I was doing before: rather than loop over values and applying each bitmask to each value, I will apply a single bitmask to _every_ value in a single pass. This requires multiple iterations over the values, but the total number of iterations between input values and bitmask values will not change.

My first thought was to partition the remaining set each time, but that only works for checking the first bit. After the first bit is processed, the two sets of values we operate on are changed.

The first pass at the solution did not yield the expected result. I used the sample data given in the puzzle description for further testing and realized that I had issues with integer division. When comparing whether the count of 1s to check whether it was more or less than half the remaining values, I was performing integer division on the length of the vector. This was sometimes causing the program to keep the opposite of the values that it should for the next iteration.

## Commentary
One interesting thing about the puzzles for day 3 is that my solution for the first part did not flow as neatly into the second part like it did for the other puzzles. I had to refactor my solution for part 1 to make the code reusable for part 2. My gut reaction was that it was more efficient to check each digit on a given value before moving on to the next value, but it is the same number of total iterations whether you check all bits before moving to the next value or check the one bit across all values.

The code could be improved by combining the `find_oxy_rating` and `find_co2_rating` functions. They only differ in the comparison that they perform to filter values.