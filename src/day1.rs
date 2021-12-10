use std::fs;

pub fn puzzle1(filename: &str) {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    let mut previous;
    let (mut current, mut counter) = (0, 0);

    for line in contents.lines() {
        previous = current;
        current = line.parse().unwrap();
        if current > previous && previous != 0 {
            counter += 1;
        }
    }

    println!("{}", counter);
}

pub fn puzzle2(filename: &str) {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    let mut previous: [i32; 3] = [0; 3];
    let mut current: [i32; 3] = [0; 3];
    let mut counter = 0;

    let mut current_sum;
    let mut previous_sum;

    for line in contents.lines() {
        previous[0] = current[0];
        previous[1] = current[1];
        previous[2] = current[2];

        current[0] = current[1];
        current[1] = current[2];
        current[2] = line.parse().unwrap();

        if previous[0] > 0 {
            current_sum = current[0] + current[1] + current[2];
            previous_sum = previous[0] + previous[1] + previous[2];

            if current_sum > previous_sum {
                counter += 1;
            }
        }
    }

    println!("{}", counter);
}

