use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./src/input.txt") {
        let mut score_part_one: i64 = 0;
        let mut score_part_two: i64 = 0;
        for line in lines {
            if let Ok(c_line) = line {
                let split = c_line.split_whitespace().collect::<Vec<&str>>();
                let opponents_move = split[0];
                let my_move = split[1];
                println!("Opponents move: {}\nMy move: {}", opponents_move, my_move);

                if (opponents_move == "A" && my_move == "X")
                 || (opponents_move == "B" && my_move == "Y")
                 || (opponents_move == "C" && my_move == "Z") {
                    println!("Draw");
                    score_part_one += 3 + get_shape_score(my_move);
                } else if (opponents_move == "A" && my_move == "Z")
                || (opponents_move == "B" && my_move == "X")
                || (opponents_move == "C" && my_move == "Y") {
                    println!("Loss");
                    score_part_one += 0 + get_shape_score(my_move);
                } else if (opponents_move == "A" && my_move == "Y")
                || (opponents_move == "B" && my_move == "Z")
                || (opponents_move == "C" && my_move == "X") {
                    println!("Win");
                    score_part_one += 6 + get_shape_score(my_move);
                } else {
                    panic!("Unknown hand")
                }

                if my_move == "X" {
                    println!("I lose");
                    score_part_two += 0 + get_shape_score(select_losing_shape(opponents_move));
                } else if my_move == "Y" {
                    println!("I draw");
                    score_part_two += 3 + get_shape_score(select_drawing_shape(opponents_move));
                } else if my_move == "Z" {
                    println!("I win");
                    score_part_two += 6 + get_shape_score(select_winning_shape(opponents_move));
                } else {
                    panic!("Unknown move")
                }
            }
        }
        println!("Final Score: {}", score_part_one);
        println!("Final Score: {}", score_part_two);
    }
}

fn get_shape_score (shape: &str) -> i64 {
    if shape == "X" {
        return 1
    } else if shape == "Y" {
        return 2
    } else if shape == "Z" {
        return 3
    } else {
        panic!("Unknown shape: {}", shape)
    }
}

fn select_winning_shape (shape: &str) -> &str {
    if shape == "A" {
        return "Y"
    } else if shape == "B" {
        return "Z"
    } else if shape == "C" {
        return "X"
    } else {
        panic!("Opponent played unknown shape")
    }
}

fn select_losing_shape (shape: &str) -> &str {
    if shape == "A" {
        return "Z"
    } else if shape == "B" {
        return "X"
    } else if shape == "C" {
        return "Y"
    } else {
        panic!("Opponent played unknown shape")
    }
}

fn select_drawing_shape (shape: &str) -> &str {
    if shape == "A" {
        return "X"
    } else if shape == "B" {
        return "Y"
    } else if shape == "C" {
        return "Z"
    } else {
        panic!("Opponent played unknown shape")
    }
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