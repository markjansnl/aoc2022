#!/bin/bash

set -e

source session
if [ -z "$1" ]; then
    DAY_LONG=$(date '+%d')
else
    DAY_LONG=$1
fi
DAY_SHORT=$(echo $DAY_LONG | sed 's/^0*//')
FOLDER=day$DAY_LONG

cargo new --lib $FOLDER
cd $FOLDER/src
echo 'pub mod input;' > lib.rs
cat > input.rs << EOF
pub const EXAMPLE: &str = include_str!("input/example.txt");

pub const USER: &str = include_str!("input/user.txt");

EOF
mkdir input
touch input/example.txt
curl "https://adventofcode.com/2022/day/$DAY_SHORT/input" --cookie "session=$SESSION" | perl -pe 'chomp if eof' > input/user.txt
mkdir bin
cat > bin/part1.rs << EOF
use $FOLDER::input;

fn f(input: &str) -> usize {
    0
}

fn main() {
    println!("{}", f(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(0, f(input::EXAMPLE));
}
EOF
