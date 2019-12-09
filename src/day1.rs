use std::fs::File;
use std::io::{BufRead, BufReader};

// Advent of Code 2019
// Day 1
pub fn print_fuel() -> std::io::Result<()> {
    println!("Day 1");

    let file = File::open("input/day1_input_paul.txt").expect("Could not open input for day1");
    let br = BufReader::new(file);
    let mut total: f64 = 0.0;

    for line in br.lines() {
        let line = line?;
        let n: f64 = line.parse().unwrap();
        let m: f64 = (n / 3.0).floor() - 2.0;
        total = total + m;
    }

    println!("Total Fuel: {}", total);
    Ok(())
}

fn get_lines(file_path: &str) -> BufReader<File> {
    let file = File::open(file_path).expect("Could not open input for day1");
    BufReader::new(file)
}

fn calc_fuel(mass: f32) -> f32 {
    let f = (mass / 3.0).floor() - 2.0;
    if f <= 0.0 {
        0.0
    } else {
        f + calc_fuel(f)
    }
}

pub fn print_fuel2() {
    let new_fuel_sum: f32 = get_lines("input/day1_input_paul.txt")
        .lines()
        .map(|l| l.unwrap().parse::<f32>().unwrap())
        .map(|m| calc_fuel(m))
        .sum();

    println!("New Total Fuel: {}", new_fuel_sum);
}
