mod day1;
mod day2;
mod day2_paul;

fn main() -> Result<(), std::io::Error> {
    println!("Hello, Advent of Code 2019!");
    println!("===========================");
    day1::print_fuel().expect("Expecting number");
    day1::print_fuel2();

    println!("Day 2");
    println!("Leftmost intcode: {}", day2::print_intcode(12, 2));
    let tup = day2::find(19690720);
    println!(
        "{:?} and (100 * noun + verb) = {}",
        tup,
        100 * tup.0 + tup.1
    );

    println!("Day 2 - Paul");
    let (noun, verb) = day2_paul::find_noun_verb(19690720);
    println!("100 * noun + verb = {}", 100 * noun + verb);

    Ok(())
}
