const INPUT: &'static str = include_str!("input");

#[allow(unused)]
fn bit_len(input: &str) -> usize {
    input.lines().next().unwrap().len()
}

#[cfg(feature = "part1")]
fn main() {
    let mut gamma = 0;
    let mut epsilon = 0;
    let line_count = INPUT.lines().count();
    for index in 0..bit_len(INPUT) {
        let one_common = INPUT.lines()
            .filter(|line| line.as_bytes()[index] == '1' as u8)
            .count() * 2 > line_count;
        gamma = (gamma << 1) | one_common as usize;
        epsilon = (epsilon << 1) | (!one_common) as usize;
    }
    dbg!(gamma * epsilon);
}

#[cfg(not(feature = "part1"))]
fn main() {
    let oxygen = scrub('1', |a, b| a >= b);
    let co2 = scrub('0', |a, b| a <= b);
    dbg!(oxygen);
    dbg!(co2);
    dbg!(oxygen * co2);
}

fn scrub(pref: char, cmp: fn(usize, usize) -> bool) -> usize {
    let pref = pref as u8;
    let mut lines: Vec<&str> = INPUT.lines().collect();
    let mut index = 0;
    while lines.len() > 1 {
        let cutoff = cmp(
            lines.iter()
                .filter(|line| line.as_bytes()[index] == pref)
                .count() * 2,
            lines.len(),
        );
        lines.retain(|line| (line.as_bytes()[index] == pref) == cutoff);
        index += 1;
    }
    usize::from_str_radix(lines.remove(0), 2).unwrap()
}