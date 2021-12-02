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
