use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::path::Path;

struct Elf {
    num_of_calories: u64,
}

fn main() {
    let mut elves: Vec<Elf> = Vec::new();

    if let Ok(lines) = read_lines("./src/input") {
        let mut current_num_of_calories = 0;
        let mut current_item: Vec<u64> = Vec::new();
        for line in lines {
            if let Ok(item) = line {
                if item.len() == 0 {
                    for item in &current_item {
                        current_num_of_calories += item
                    }
                    elves.push(Elf {
                        num_of_calories: current_num_of_calories,
                    });
                    current_num_of_calories = 0;
                    current_item.clear();
                } else {
                    current_item.push(item.parse::<u64>().unwrap())
                }
            }
        }
    }

    elves.sort_by(|a, b| b.num_of_calories.cmp(&a.num_of_calories));
    println!("Most calories carried: {}", elves[0].num_of_calories);
    println!(
        "Top 3 calories: {}",
        elves[0].num_of_calories + elves[1].num_of_calories + elves[2].num_of_calories
    );
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
