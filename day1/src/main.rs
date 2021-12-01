const INPUT: &'static str = include_str!("input");

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
