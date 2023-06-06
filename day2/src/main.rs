use std::{fs::File, io::{BufReader, BufRead, Read}};

fn part1(reader: &mut BufReader<File>) {
    let score: i16 = reader.by_ref().lines()
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

fn part2() {

}

fn main() {
    let f = File::open("./input.txt").unwrap();
    let mut reader = BufReader::new(f);

    part1(&mut reader);
    part1(&mut reader);
}

fn get_turn_point(turn: &str) -> Option<(i8, i8)> {
    match turn {
        "A" | "X" => Some((1,0)), //rock
        "B" | "Y" => Some((2,2)), //paper
        "C" | "Z" => Some((3,1)), //scissor
        _ => None
    }
}
