const INPUT: &'static str = include_str!("input");

#[cfg(feature = "part1")]
fn main() {
    let mut lines = INPUT.lines()
        .map(|line| line.parse::<usize>().unwrap());
    let mut v = lines.next().unwrap();
    let mut increases = 0;
    for line in lines {
        if line > v {
            increases += 1;
        }
        v = line;
    }

    println!("{} increases", increases);
}

#[cfg(not(feature = "part1"))]
fn main() {
    let lines: Vec<usize> = INPUT
        .lines()
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    let mut v: usize = lines[0..=2].iter().sum();
    let mut increases = 0;
    for y in 0..lines.len()-2 {
        let sum: usize = lines[y..=y+2].iter().sum();
        if sum > v {
            increases += 1;
        }
        v = sum;
    }

    println!("{}", increases);
}