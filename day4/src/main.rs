use std::ops::Deref;

const INPUT: &'static str = include_str!("input");

#[derive(Debug, Copy, Clone)]
struct Check<T: Copy> {
    checked: bool,
    value: T,
}

impl<T: Copy> Deref for Check<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
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

fn main() {
    let mut bingo = read_bingo(INPUT);

    for num in bingo.numbers {
        for board in &mut bingo.boards {
            board.check(num);
            if board.is_won() {
                let sum: usize = board.0.iter()
                    .map(|row| row.iter()
                        .filter(|check| !check.checked)
                        .map(|check| check.value).sum::<usize>())
                    .sum();
                dbg!(sum);
                dbg!(num);
                dbg!(sum * num);
                return;
            }
        }
    }
}
