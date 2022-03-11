The goal of the [puzzle](https://adventofcode.com/2021/day/2) is to read input for movement and track horizontal position and depth of a submarine.

The input is in the form
```
forward 5
down 5
forward 8
up 3
down 8
forward 2
```

## Part I
Position and depth both start at 0. Moving `down` _increases_ depth while moving `up` _decreases_ depth.

Follow the steps in the [input](https://adventofcode.com/2021/day/2/input). Multiply the final position by the final depth to get the puzzle answer.

1. Parse input into lines
2. Parse each line into an instruction
3. Follow instructions while updating position and depth
4. Output final result

## Part II
The instructions remain the same, but rather than `up` and `down` controlling depth, they adjust a new `aim` value. A `forward` instruction increases the horizontal position of the submarine _and_ adjusts the depth by the value multiplied by the current `aim` value.

It's a simple adjustment to re-iterate the instructions that were created in Part I using a separate `match` statement for adjusting the position and depth.