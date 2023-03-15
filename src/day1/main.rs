use std::fs::File;
use std::io::Read;

fn main() {
    // Use a buffer to read the input file
    let mut buffer = String::new();
    // Open the file
    let mut file = File::open("src/day1/resources/input.txt").expect("Unable to open file");
    // Read the file into the buffer
    file.read_to_string(&mut buffer).expect("Unable to read file");

    let mut current_sum: u32 = 0;
    let mut max_sums: Vec<u32> = vec![0, 0, 0];

    for line in buffer.lines() {
        if line.trim().is_empty() {
            // The current list has ended, so update the max sums if necessary
            if current_sum > max_sums[0] {
                max_sums[2] = max_sums[1];
                max_sums[1] = max_sums[0];
                max_sums[0] = current_sum;
            } else if current_sum > max_sums[1] {
                max_sums[2] = max_sums[1];
                max_sums[1] = current_sum;
            } else if current_sum > max_sums[2] {
                max_sums[2] = current_sum;
            }
            // Reset the current sum for the next list
            current_sum = 0;
        } else {
            // Add the number in the current line to the current sum
            let number = line.parse::<u32>().unwrap();
            current_sum += number;
        }
    }

// Sum the top 3 max sums
    let sum = max_sums.iter().sum::<u32>();

    println!("Sum of top 3 lists: {}", sum);
    println!("Best list: {}", max_sums[0])
}

