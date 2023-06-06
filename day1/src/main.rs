use std::{fs::{File, self}, io::{BufRead, BufReader}, cmp::Reverse};

fn read_file() -> BufReader<File> {
    let f = File::open("./input2.txt").expect("error");
    BufReader::new(f)
}

fn main() {
    day1();
    day2();
}

fn day1() {
    let reader = read_file();

    let mut max_calories = 0;
    let mut sum = 0;
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => continue,
        };

        if line.is_empty() {
            if sum > max_calories {
                max_calories = sum;
            }
            sum = 0; // reset sum to 0
        } else {
            let cal = line.parse::<u32>().expect("not a number");
            sum += cal;
        }
    }
}

fn day2() {
    let mut res = fs::read_to_string("input.txt").unwrap()
        .split("\n\n")
        .map(|group| group.lines().map(|c| c.parse::<u32>().unwrap())
        .sum::<u32>())
        .collect::<Vec<u32>>();

    res.sort_unstable_by_key(|w| Reverse(*w));

    println!("total: {}", res[0] + res[1] + res[2]);
}
