#!/bin/sh

cargo build
cat day-01-input.txt | ./target/debug/day-01 > day-01-output.txt
