use std::fs;

fn main() {

    let filename = "inputs/day1_1.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let (mut previous, mut current, mut counter) = (0, 0, 0);

    for line in contents.lines() {
        previous = current;
        current = line.parse().unwrap();
        if current > previous && previous != 0 {
            counter += 1;
        }
    }

    println!("{}", counter);
}
