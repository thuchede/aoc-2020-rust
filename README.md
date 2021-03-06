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
### Day 8
- implementing a enum struct to hold instruction was a good hindsight
- really easy to implements needed trait afterwards
- IntCode v2 incoming?
### Day 9
- (x..y) is nice to quickly generate a range (or getting a slice with [x..y])
- small example worked fine for part 2 but stack overflow'd on puzzle input 
### Day 10
- Can't find the formula for picking out adapters combinaisons
- looks like we should pick (n in m) but need to remove some x that wouldn't match a difference > 3
- => `2^(n-1)-max(2^(n-3)-1, 0)` (also cf. tribonacci sequence)
### Day 11
- cost of `clone()` ?
- turns out by passing ref instead of cloning time goes down
- pre optim : p1 ~49sec p2 ~36sec
- post optim : p1 ~1sec p2 ~2sec
### Day 12
- can't extends struct, you either use composition, redeclare another struct or add optional field to base struct