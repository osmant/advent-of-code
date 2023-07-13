fn main() {
    let str = include_str!("input.txt");

    println!("{}", part1(str));
    println!("{}", part2(str));
}


fn part1(input : &str) -> usize {
    let acc = input
        .lines()
        .map(str::split_whitespace)
        .map(|mut str| {
            let command = str.next().unwrap();
            let n = str.next().unwrap().parse::<usize>().unwrap();
            (command, n)
        })
        .fold((0, 0), |mut acc, (command, n)| {
            match command {
                "forward" => {
                    acc.0 += n;
                },
                "up" => {
                    acc.1 -= n;
                },
                "down" => {
                    acc.1 += n;
                },
                _ => unreachable!()
            }
            acc
        });

    acc.0 * acc.1
}

fn part2(input : &str) -> usize {

    let initial_pos = Position { depth : 0, horizontal: 0, aim: 0 };

    let acc = input
        .lines()
        .map(str::split_whitespace)
        .map(|mut str| {
            let command = str.next().unwrap();
            let n = str.next().unwrap().parse::<usize>().unwrap();
            (command, n)
        })
        .fold(initial_pos, |mut acc, (command, n)| {
            match command {
                "forward" => {
                    acc.horizontal += n;
                    acc.depth += acc.aim * n;
                },
                "up" => {
                    acc.aim -= n;
                },
                "down" => {
                    acc.aim += n;
                },
                _ => unreachable!()
            }
            acc
        });

    acc.horizontal * acc.depth
}

struct Position
{
    depth: usize,
    horizontal: usize,
    aim: usize,
}