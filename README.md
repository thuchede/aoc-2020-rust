# AOC 2020

## What ?
Advent of Code is an Advent calendar of small programming puzzles. I'm attempting to solve all puzzles in rust this year

## Why?
Last year I tried to solve as much as I could to learn about Golang and it was so much fun. 

## Learnings

### Day 1 
- recursion is as fun as always
- Rust does not seem to have tail call optimization
- Option<T> is neat
- Rust's built-in testing seems as nice as Go's

### Day 2 
- rust std regex crate does not support lookahead
- simple loop might do the trick simpler than regex for this one

### Day 3
- Part 2 was tricky: need to skip some iteration based on x
### Day 4
- Don't give me a language with regex or I might use them everywhere
- manipulation between String and str is sometimes cumbersome (but not complicated by the language)
### Day 5
- Easy because of puzzle title who gave away the trick
- compute_seat_id_from_boarding_pass might have been unnecessary and computation could have been done directly on 10 bits
### Day 6
- rust std is HUGE and filled with a lot of things, it's hard to know exactly what you're looking for
- golang was so bare there often was only one way to to thing: it's not the case here
### Day 7
- NEVER EVER EVER compile regex in a loop => performance drop
Regex compiled each fn call : 
```sh
❯ time ./target/debug/aoc-2020-rust
Day7 - 1 : 265
./target/debug/aoc-2020-rust  98,38s user 0,46s system 99% cpu 1:39,48 total
```
Regex passed as param :
```sh
❯ time ./target/debug/aoc-2020-rust
Day7 - 1 : 265
./target/debug/aoc-2020-rust  0,73s user 0,00s system 99% cpu 0,740 total
```
- `.clone()` everywhere might not be the best way, might be interesting to visualize the amount ou memory
- beginning to get a handle on the difference between `T`, `&T` and `&mut T` :)