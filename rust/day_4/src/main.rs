use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn main() {
    let assignments = read_lines("./src/input.txt")
        .unwrap()
        .map(|line| parse_assignment(&line.unwrap()))
        .collect::<Vec<Assignment>>();

    let overlapping = assignments
        .iter()
        .filter(|assignment| is_either_fully_contained(assignment))
        .collect::<Vec<&Assignment>>();

    println!("Number of completely overlapping assignments: {}", overlapping.len());

    let partial_overlapping = assignments
        .iter()
        .filter(|assignment| assignment_have_overlapping_ranges(assignment))
        .collect::<Vec<&Assignment>>();

    println!("Number of partially overlapping assignments: {}", partial_overlapping.len());
}

fn parse_assignment(line: &String) -> Assignment {
    let ranges = line.split(",").collect::<Vec<&str>>();

    if ranges.len() != 2 {
        panic!("Each row must contain two assignments. Given line contains a different amount.\nAmount '{}'\nLine '{}'\n", ranges.len(), line);
    }

    let range_one = ranges[0].split("-").collect::<Vec<&str>>();
    let range_two = ranges[1].split("-").collect::<Vec<&str>>();

    if range_one.len() != 2 {
        panic!(
            "A range must contain two parts.\nAmount '{}'\n",
            range_one.len()
        );
    }

    if range_two.len() != 2 {
        panic!(
            "A range must contain two parts.\nAmount '{}'\n",
            range_two.len()
        );
    }

    let start_range_one = range_one[0].parse::<u64>().unwrap();
    let end_range_one = range_one[1].parse::<u64>().unwrap();
    if start_range_one > end_range_one {
        panic!(
            "Start of range one must be before end\nStart: '{}'\nEnd: '{}'",
            start_range_one, end_range_one
        );
    }

    let start_range_two = range_two[0].parse::<u64>().unwrap();
    let end_range_two = range_two[1].parse::<u64>().unwrap();
    if start_range_two > end_range_two {
        panic!(
            "Start of range two must be before end\nStart: '{}'\nEnd: '{}'",
            start_range_two, end_range_two
        );
    }

    return Assignment {
        assignment_one: Range {
            start: start_range_one,
            end: end_range_one,
        },
        assignment_two: Range {
            start: start_range_two,
            end: end_range_two,
        },
    };
}

fn is_either_fully_contained(assignment: &Assignment) -> bool {
    let range_one = &assignment.assignment_one;
    let range_two = &assignment.assignment_two;

    return is_first_range_contained_by_second(range_one, range_two)
        || is_first_range_contained_by_second(range_two, range_one);
}

fn assignment_have_overlapping_ranges (assignment: &Assignment) -> bool {
    let range_one = &assignment.assignment_one;
    let range_two = &assignment.assignment_two;

    return is_start_first_range_inside_second_range(range_one, range_two)
        || is_end_first_range_inside_second_range(range_one, range_two)

        || is_start_first_range_inside_second_range(range_two, range_one)
        || is_end_first_range_inside_second_range(range_two, range_one)
        ;
}

fn is_first_range_contained_by_second(first_range: &Range, second_range: &Range) -> bool {
    let start_first_range_inside_second_range: bool = is_start_first_range_inside_second_range(first_range, second_range);
    let end_first_range_inside_second_range: bool = is_end_first_range_inside_second_range(first_range, second_range);

    return start_first_range_inside_second_range && end_first_range_inside_second_range;
}

fn is_start_first_range_inside_second_range (first_range: &Range, second_range: &Range) -> bool {
    return second_range.start <= first_range.start && first_range.start <= second_range.end;
}

fn is_end_first_range_inside_second_range (first_range: &Range, second_range: &Range) -> bool {
    return second_range.start <= first_range.end && first_range.end <= second_range.end;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[derive(PartialEq, Debug)]
struct Assignment {
    assignment_one: Range,
    assignment_two: Range,
}

#[derive(PartialEq, Debug)]
struct Range {
    start: u64,
    end: u64,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_either_fully_contained_false_for_disjointed() {
        let disjointed = Assignment {
            assignment_one: Range { start: 2, end: 4 },
            assignment_two: Range { start: 6, end: 8 },
        };
        assert_eq!(is_either_fully_contained(&disjointed), false);
    }

    #[test]
    fn test_is_either_fully_contained_false_for_range_one_overlapping_range_two() {
        let range_one_overlap_range_two = Assignment {
            assignment_one: Range { start: 2, end: 4 },
            assignment_two: Range { start: 5, end: 8 },
        };
        assert_eq!(
            is_either_fully_contained(&range_one_overlap_range_two),
            false
        );
    }

    #[test]
    fn test_is_either_fully_contained_false_for_range_two_overlapping_range_one() {
        let range_two_overlap_range_one = Assignment {
            assignment_one: Range { start: 2, end: 4 },
            assignment_two: Range { start: 5, end: 8 },
        };
        assert_eq!(
            is_either_fully_contained(&range_two_overlap_range_one),
            false
        );
    }

    #[test]
    fn test_is_either_fully_contained_true_for_range_one_inside_range_two() {
        let range_one_contained_in_range_two = Assignment {
            assignment_one: Range { start: 4, end: 6 },
            assignment_two: Range { start: 3, end: 8 },
        };
        assert_eq!(
            is_either_fully_contained(&range_one_contained_in_range_two),
            true
        );
    }

    #[test]
    fn test_is_either_fully_contained_true_for_range_two_inside_range_one() {
        let range_two_contained_in_range_one = Assignment {
            assignment_one: Range { start: 1, end: 4 },
            assignment_two: Range { start: 2, end: 3 },
        };
        assert_eq!(
            is_either_fully_contained(&range_two_contained_in_range_one),
            true
        );
    }

    #[test]
    fn test_parse_assignment() {
        let expected = vec![
            Assignment {
                assignment_one: Range { start: 2, end: 4 },
                assignment_two: Range { start: 6, end: 8 },
            },
            Assignment {
                assignment_one: Range { start: 2, end: 3 },
                assignment_two: Range { start: 4, end: 5 },
            },
            Assignment {
                assignment_one: Range { start: 5, end: 7 },
                assignment_two: Range { start: 7, end: 9 },
            },
            Assignment {
                assignment_one: Range { start: 2, end: 8 },
                assignment_two: Range { start: 3, end: 7 },
            },
            Assignment {
                assignment_one: Range { start: 6, end: 6 },
                assignment_two: Range { start: 4, end: 6 },
            },
            Assignment {
                assignment_one: Range { start: 2, end: 6 },
                assignment_two: Range { start: 4, end: 8 },
            },
        ];

        let actual = read_lines("./src/test-input.txt")
            .unwrap()
            .map(|line| parse_assignment(&line.unwrap()))
            .collect::<Vec<Assignment>>();

        assert_eq!(actual, expected);
    }
}
