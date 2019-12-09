// Advent of Code 2019
// Day 2
pub fn print_intcode(x: usize, y: usize) -> usize {
    let code = [
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 13, 1, 19, 1, 19, 10, 23, 2, 10, 23, 27,
        1, 27, 6, 31, 1, 13, 31, 35, 1, 13, 35, 39, 1, 39, 10, 43, 2, 43, 13, 47, 1, 47, 9, 51, 2,
        51, 13, 55, 1, 5, 55, 59, 2, 59, 9, 63, 1, 13, 63, 67, 2, 13, 67, 71, 1, 71, 5, 75, 2, 75,
        13, 79, 1, 79, 6, 83, 1, 83, 5, 87, 2, 87, 6, 91, 1, 5, 91, 95, 1, 95, 13, 99, 2, 99, 6,
        103, 1, 5, 103, 107, 1, 107, 9, 111, 2, 6, 111, 115, 1, 5, 115, 119, 1, 119, 2, 123, 1, 6,
        123, 0, 99, 2, 14, 0, 0,
    ];

    let mut counter: usize = 0;
    let mut mutated = code;
    let mut answer = 0;

    mutated[1] = x;
    mutated[2] = y;

    for _x in code.iter() {
        let opcode = mutated[counter];

        match opcode {
            1 => {
                let first = mutated[counter + 1];
                let second = mutated[counter + 2];
                let sum = mutated[first] + mutated[second];
                let index = mutated[counter + 3];
                mutated[index] = sum;
            }
            2 => {
                let first = mutated[counter + 1];
                let second = mutated[counter + 2];
                let product = mutated[first] * mutated[second];
                let index = mutated[counter + 3];
                mutated[index] = product;
            }
            99 => {
                answer = mutated[0];
                break;
            }
            _ => println!("Something went wrong."),
        }
        counter = counter + 4;
    }
    return answer;
}

pub fn find(z: usize) -> (usize, usize) {
    let mut x = 0;
    let mut y = 0;
    for i in 0..100 {
        x = i;
        for j in 0..100 {
            y = j;
            let test = print_intcode(x, y);
            if test == z {
                println!("z is {}, x is {}, y is {} and f(x,y) is {}", z, x, y, test);
                return (x, y);
            }
        }
    }
    return (x, y);
}
