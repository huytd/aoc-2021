[Advent of Code 2021](https://adventofcode.com/2021)

To start a new day, run:

```
make new day=05
```

## Daily Logs

### Day 01

For part one, just iterate through the input list and see if the current number is larger than the previous one.

For part two, we do the same, but this time, instead of comparing each number, we compare the `sum = i + (i+1) + (i+2)`.

### Day 02

Today problem can be solved by iterate through the input commands and do what it said for each command, the `x`, `y` and `aim` values
are created and modified as we go through the list.

### Day 03

Instead of reading the input data as a list of numbers (represented in binary format), I got lazy, so I just read them as a string.

Instead of implementing an actual bit counting algorithm, I just count the number of characters :D

In part two, my assumption from part one bited me :( I only count the number of 1 bits, if `count_1 > input.len() / 2` so I assume
the number column has more 1 bits than 0s. But working with integers mean `input.len() / 2` will get rounded down, for example `7 / 2 = 3`
and the algorithm went wrong from that point.

So I had to count the number of zeroes too. The rest doesn't have much to say.

### Day 04

Oh I love Rust's trait system for this challenge. Implementing the methods like `find_number` or `is_board_win` at `Board` level and call them as
we iterate through the board list helped the algorithm looks cleaner, but did not helps much with the time complexity.

One mistake I made is using [`slice::binary_search`](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search) for searching the number
in each row, they're not sorted, and I forgot, so the built in binary search would not works here. 30 min wasted :D

My solution might not elegance enough, but it works. After reading the data into the list of boards, we iterate through the list of draws, for each
number, find that number in all boards, if we found it, we mark the number on that board. Finally, check if the board is finished and get the result.

Since I use a second grid to mark the cells on the board, 1 is marked and 0 is not. To check if the board is finished or not, I just need to calculate
the sum of each row and columns, if there is one that larger than 5 (total number of row, or column), that board is finished.

For part 2, use a number to keep track of finished boards, instead of returning the result immediatelly, we increase a counter, and continue the algorithm.
Eventually, if all boards are finished, we can get the result of the last finished board.

### Day 05

There are many ways to solve today challenge, but I'll go for a simple one: For each line from the input, generate all points between the start and end point.

For horizontal and vertical lines, generating points is easy:

```
for x from start_x -> end_x:
  new_point = (x, start.y)

or

for y from start_y -> end_y:
  new_point = (start.x, y)
```

For diagonal lines, any [line drawing algorithm](https://en.m.wikipedia.org/wiki/Line_drawing_algorithm) can be used, like Bresenham. But in this problem, all diagonal lines are just the 45 degree lines, so
using the naive line drawing algorithm is enough.

When we have the list of all points, find the duplicate points and count the number of them.

### Day 06

Interesting problem today. The straight forward approach is iterate through the list of fish, decrease their counter and add a new fish into the list as they spawn.

This does not works in part 2, for obvious reason, the fishes grow exponentially.

If we look at the input data the different way, instead of a list of numbers, think about a hash map where each key is the count of how many days left for that fish.

```
Input: 3, 4, 3, 1, 2

Hashmap:
[key = 0; value = 0]
[key = 1; value = 1]
[key = 2; value = 1]
[key = 3; value = 2]
[key = 4; value = 0]
[key = 5; value = 0]
[key = 6; value = 0]
[key = 7; value = 0]
[key = 8; value = 0]
```

Now, for each generation `n`, we decrease the value of `hashmap[n]` and increase the `hashmap[n-1]` by `1`. That mean we just rotated the values by 1 position.

So, if we keep rotate until we reach the end of the generation, and add new fishes as they spawn to the list, the sum of all entries in the hashmap is the number we're looking for.
