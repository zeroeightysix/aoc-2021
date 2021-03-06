const INPUT: &'static str = include_str!("input");

#[cfg(feature = "part1")]
fn main() {
    let (x, y) = INPUT
        .lines()
        .fold((0, 0), |(x, y), line| {
            let (insn, num) = line.split_at(line.find(' ').unwrap());
            let num = num.trim().parse::<isize>().unwrap();
            match insn {
                "forward" => (x + num, y),
                "down" => (x, y + num),
                "up" => (x, y - num),
                _ => panic!("invalid insn")
            }
        });

    dbg!(x * y);
}

#[cfg(not(feature = "part1"))]
fn main() {
    let (x, y, _) = INPUT
        .lines()
        .fold((0, 0, 0), |(x, y, aim), line| {
            let (insn, num) = line.split_at(line.find(' ').unwrap());
            let num = num.trim().parse::<isize>().unwrap();
            match insn {
                "forward" => (x + num, y + aim * num, aim),
                "down" => (x, y, aim + num),
                "up" => (x, y, aim - num),
                _ => panic!("invalid insn")
            }
        });

    dbg!(x * y);
}