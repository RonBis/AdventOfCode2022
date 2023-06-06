use std::{fs::File, io::{BufReader, BufRead}};

fn get_turn_point(turn: &str) -> Option<(i8, i8)> {
    match turn {
        "A" | "X" => Some((1,0)), //rock
        "B" | "Y" => Some((2,2)), //paper
        "C" | "Z" => Some((3,1)), //scissor
        _ => None
    }
}

fn part1(f: &File) {
    let score: i16 = BufReader::new(f)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let round: Vec<&str> = line.split_ascii_whitespace().collect();
            let (_elf_point, elf_strength) = get_turn_point(round[0]).unwrap();
            let (my_point, my_strength) = get_turn_point(round[1]).unwrap();

            if (elf_strength+1) % 3 == my_strength % 3 {
                // I lost
                (my_point + 0) as i16
            } else if elf_strength % 3  == (my_strength+1) % 3 {
                // I won
                (my_point + 6) as i16
            } else {
                // draw
                (my_point + 3) as i16
            }
        }).sum();

    println!("{}", score);
}

fn part2(f: &File) {
    let score: i16 = BufReader::new(f)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let round = line.as_bytes();

            /* returns
            // 0 * 3 => lose
            // 1 * 3 => draw
            // 2 * 3 => win */
            let my_strategy = ((2 - (b'Z' - round[2])) * 3) as i16;
            let elf_turn = (2 - (b'C' - round[0])) as i16;
            let my_point = match my_strategy {
                0 => (elf_turn + 2) % 3 + 1, // (elf_turn - 1) can result in negative, so i used (elf_turn + 2)
                3 => elf_turn + 1,
                6 => (elf_turn + 1) % 3 + 1,
                _ => 0,
            };

            my_point + my_strategy
        }).sum();

    println!("{}", score);
}

fn main() {
    let f = File::open("./input.txt").unwrap();

    // part1(&f);
    part2(&f);
}
