# aoc2023
Advent of Code 2023

# Notes

## Day 10 Part 01

Unfortunately my recursive implementation on Day 10 Part 01 nailed the stack on WSL2 NixOS. In order to get this to work I had to run (I couldn't be bothered to implement an iterative alternative):

`RUST_MIN_STACK=104857600 cargo test`