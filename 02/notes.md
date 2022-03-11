The goal of the [puzzle](https://adventofcode.com/2021/day/2) is to read input for movement and track horizontal position and epth of a submarine.

The input is in the form
```
forward 5
down 5
forward 8
up 3
down 8
forward 2
```

Position and depth both start at 0. Moving `down` _increases_ depth while moving `up` _decreases_ depth.

Follow the steps in the [input](https://adventofcode.com/2021/day/2/input). Multiply the final position by the final depth to get the puzzle answer.

1. Parse input into lines
2. Parse each line into an instruction
3. Follow instructions while updating position and depth
4. Output final result