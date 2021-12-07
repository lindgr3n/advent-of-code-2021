# advent-of-code-2021
https://adventofcode.com/ challange in rust

Trying out Rust for the advent of code challanges. Will try and document my struggle here :)

## Day 1

Easy to get started. Fast to install and get going. Good rust extension to VSCode.
Like how easy it was to get unit tests up and running.

Initial i wanted to make a day1.rs, day2.rs etc. but did not find a way to run a specific file. So ended up creating a new project for each day.
Knowing how and what to typecasting and setting up arrays is a lot of searching the docs :) As an JS developer let is mutable by default.
Think it would be good to read the docs a bit :)

## Day 2

Easier then Day 1 and 10 times faster now that i had a base from day 1. Struggled most with understanding when passing a mutable array. 
Thats not allowed to be passed a second time if one is not cloning the varible.

Feeel like one could make this a bit easier. Or should you clone the variable.

```
let lines = contents.lines().collect::<Vec<_>>();
let result_part_one = calculate_position(lines.clone());
let result_part_two = calculate_position_with_aim(lines.clone());
```

Tried to be fancy and using a enum for direction checking. Did not go so well. Need to try that more in the future.

## Day 3

This must be the most ugliest solution. Hard to think in another way once you have commited. But hey it works :)
Filtering was nice to understand, stange syntax using `| x |`.

Things I need to read up more on 
* `Vec`
* `&` why when
* `unwrap()`
* `let mut tmp: Vec<i32> = [].to_vec();` Feels like there should be another way to approach this
* `.iter()` and `.into_iter()`


## Day 4


## Day 5

## Day 6

First part was just basic and brute force. That was not optimal in step two where the result never finished... A bit short on time so looked for inspiration and found the solution for part two,
Seeing the solution it was kinda "easy" we do not need a index for each fish. We will always have a maximum of nine fishes that we can rotate around. Was done in ms instead of never finished :)
Don't think i would have thought about that approach without looking it up.

## Day 7

Kinda straight forward solution. Just brute force it by looping. Should have made it more dynamic by calculating the maximum number of time we should loop but this was faster by just looking what the maximum value in the input was. That the maximum value we need to go down from.
My first thought was to do some kind of random selection. Go to 100 then 50 should we increase or decrease just to remove some values. In the worst case it should been as slow as my o(n) solution now.
Long time ago i thought about o(n) algorithms. 
Second part i kinda remember some kind of formula to calculate 1+2+3...n but did not remember and just made a loop :)