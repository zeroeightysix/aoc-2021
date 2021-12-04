#![feature(vec_retain_mut)]

use std::fmt::{Display, Formatter};

const INPUT: &'static str = include_str!("input");

#[derive(Debug, Copy, Clone)]
struct Check<T: Copy> {
    checked: bool,
    value: T,
}

#[derive(Debug)]
struct Board([[Check<usize>; 5]; 5]);

impl Board {
    fn is_won(&self) -> bool {
        let Board(content) = self;
        (0..5).any(|row| content[row].iter().all(|check| check.checked))
            || (0..5).any(|col| (0..5).all(|row| content[row][col].checked))
    }

    fn check(&mut self, num: usize) {
        (0..5).for_each(|row| (0..5).for_each(|col| {
            let check = &mut self.0[row][col];
            check.checked |= check.value == num;
        }))
    }

    fn unmarked_sum(&self) -> usize {
        self.0.iter()
            .map(|row| row.iter()
                .filter(|check| !check.checked)
                .map(|check| check.value).sum::<usize>())
            .sum()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (0..5)
            .map(|col| self.0[col].map(|c| format!("{}{}", if c.checked { "X" } else { "" }, c.value)).join("\t"))
            .collect::<Vec<String>>()
            .join("\n")
        )
    }
}

#[derive(Debug)]
struct Bingo {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

fn read_bingo(input: &str) -> Bingo {
    let lines: Vec<&str> = input.lines().collect();
    let numbers: Vec<usize> = lines[0]
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    let boards: Vec<Board> = lines[2..]
        .chunks(6)
        .map(|chunk| chunk[0..5].iter()
            .map(|row| row.split_whitespace()
                .map(|v| Check {
                    value: v.parse().unwrap(),
                    checked: false,
                })
                .collect::<Vec<Check<usize>>>()
                .try_into().unwrap())
            .collect::<Vec<[Check<usize>; 5]>>().try_into().unwrap())
        .map(|board| Board(board))
        .collect();
    Bingo {
        numbers,
        boards,
    }
}

#[cfg(feature = "part1")]
fn main() {
    let mut bingo = read_bingo(INPUT);

    for num in bingo.numbers {
        for board in &mut bingo.boards {
            board.check(num);
            if board.is_won() {
                let sum = board.unmarked_sum();
                dbg!(sum);
                dbg!(num);
                dbg!(sum * num);
                return;
            }
        }
    }
}

#[cfg(not(feature = "part1"))]
fn main() {
    let mut bingo = read_bingo(INPUT);

    for num in bingo.numbers {
        let len = bingo.boards.len();
        if len > 1 {
            bingo.boards.retain_mut(|board| {
                board.check(num);
                !board.is_won()
            });
        } else {
            let board = &mut bingo.boards[0];
            board.check(num);
            if board.is_won() {
                println!("{}", board);
                let sum = board.unmarked_sum();
                dbg!(sum);
                dbg!(num);
                dbg!(sum * num);
                return;
            }
        }
    }
}