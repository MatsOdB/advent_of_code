use std::fs::File;
use std::io::Read;

fn main() {
    // Use a buffer to read the input file
    let mut buffer = String::new();
    // Open the file
    let mut file = File::open("src/day1/resources/input.txt").expect("Unable to open file");
    // Read the file into the buffer
    file.read_to_string(&mut buffer).expect("Unable to read file");

    let mut max_cals: u32 = 0;
    let mut current_cals: u32 = 0;

    for line in buffer.lines() {
        // Convert the line to a number
        let number = line.parse::<u32>();
        if number.is_ok() {
            current_cals += number.unwrap();
        } else {
            if current_cals > max_cals {
                max_cals = current_cals;
            }
            current_cals = 0;
        }
    }

    println!("Max cals: {}", max_cals);
}
