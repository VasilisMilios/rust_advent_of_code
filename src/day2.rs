use std::fs;

pub fn puzzle1(filename: &str) {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    let mut horizontal = 0;
    let mut depth = 0;
    let mut iter;
    let mut direction;
    let mut units: i32;

    for line in contents.lines() {
        iter = line.split_whitespace();
        direction = iter.next().unwrap();
        units = iter.next().unwrap().
            parse().unwrap();

        match direction {
            "forward" => horizontal += units,
            "down" => depth += units,
            "up" => depth -= units,
            _ => println!("Ohoh"),
        }

        

    }
    println!("{}", horizontal*depth);
}
