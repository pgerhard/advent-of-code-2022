use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::path::Path;

#[derive(PartialEq, Debug, Clone)]
struct Rucksack {
    first_compartment: Vec<String>,
    second_compartment: Vec<String>,
}

#[derive(PartialEq, Debug)]
struct Group {
    rucksacks: Vec<Rucksack>,
}

fn main() {
    let all_lines: Vec<String> = load_lines();

    let rucksacks = all_lines
        .iter()
        .map(|all_items| parse_rucksack(all_items))
        .collect::<Vec<Rucksack>>();
    let duplicates = rucksacks
        .iter()
        .flat_map(|rucksack| find_duplicates(rucksack))
        .collect::<Vec<String>>();
    let priorities = calculate_priorities(duplicates);
    let priority_sum: i32 = priorities.iter().sum();

    println!("Priority Sum: {}", priority_sum);

    let groups = create_groups(all_lines);
    let badge_item_types = find_badge_item_types(groups);
    let badge_priorities = calculate_priorities(Vec::from_iter(badge_item_types.iter().cloned()));
    let badge_priority_sum: i32 = badge_priorities.iter().sum();

    println!("Badges priority Sum: {}", badge_priority_sum);
}

fn load_lines() -> Vec<String> {
    let mut all_lines: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            if let Ok(c_line) = line {
                all_lines.push(c_line)
            }
        }
    }
    return all_lines;
}

fn parse_rucksack(all_items: &String) -> Rucksack {
    let mut rucksack = Rucksack {
        first_compartment: Vec::new(),
        second_compartment: Vec::new(),
    };

    let num_of_items = all_items.len();
    for (pos, item) in all_items.chars().enumerate() {
        if pos < num_of_items / 2 {
            rucksack.first_compartment.push(item.to_string())
        } else {
            rucksack.second_compartment.push(item.to_string())
        }
    }

    return rucksack;
}

fn create_groups(all_lines: Vec<String>) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();

    let mut group_rucksacks: Vec<Rucksack> = Vec::new();
    for (pos, line) in all_lines.iter().enumerate() {
        let rucksack = parse_rucksack(line);
        group_rucksacks.push(rucksack);
        if (pos + 1) % 3 == 0 {
            groups.push(Group {
                rucksacks: Vec::from_iter(group_rucksacks.iter().cloned()),
            });
            group_rucksacks.clear();
        }
    }

    return groups;
}

fn find_duplicates(rucksack: &Rucksack) -> HashSet<String> {
    let mut duplicates: HashSet<String> = HashSet::new();
    for item in rucksack.first_compartment.iter() {
        if rucksack.second_compartment.contains(item) {
            duplicates.insert(item.clone());
        }
    }
    return duplicates;
}

fn find_badge_item_types(groups: Vec<Group>) -> Vec<String> {
    let mut badge_types : Vec<String> = Vec::new();
    
    for group in groups {
        if group.rucksacks.len() != 3 {
            panic!("Group must contain 3 rucksacks")
        }

        let group_items: Vec<String> = group
            .rucksacks
            .iter()
            .map(|rucksack| {
                let mut tmp: Vec<String> = Vec::new();
                tmp.append(&mut Vec::from_iter(rucksack.first_compartment.clone()));
                tmp.append(&mut Vec::from_iter(rucksack.second_compartment.clone()));
                return tmp.join("");
            })
            .collect();
        let rucksack_one  = group_items[0].chars().map(|c| c.to_string()).collect::<Vec<String>>();
        let rucksack_two = group_items[1].chars().map(|c| c.to_string()).collect::<Vec<String>>();
        let rucksack_three = group_items[2].chars().map(|c| c.to_string()).collect::<Vec<String>>();

        let mut tmp_badge_types : HashSet<String> = HashSet::new();
        for item in rucksack_one {
            if rucksack_two.contains(&item) && rucksack_three.contains(&item) {
                tmp_badge_types.insert(item.clone());
            }
        }
        for ele in tmp_badge_types {
            badge_types.push(ele);
        }
    }
    return badge_types;
}

fn calculate_priorities(duplicates: Vec<String>) -> Vec<i32> {
    let mut priorities: Vec<i32> = Vec::new();

    for duplicate in duplicates {
        if duplicate.chars().count() != 1 {
            panic!("Duplicate is not of length 1: {}", duplicate)
        }
        let ascii_value = duplicate.chars().nth(0).unwrap() as i32;
        if 97 <= ascii_value && ascii_value <= 122 {
            priorities.push(ascii_value - 96)
        } else if 65 <= ascii_value && ascii_value <= 90 {
            priorities.push(ascii_value - 38)
        } else {
            panic!(
                "Neither uppercase or lowercase ASCII value: {}",
                ascii_value
            )
        }
    }

    return priorities;
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_rucksack() {
        let expected = Rucksack {
            first_compartment: vec!["a".to_string(), "B".to_string(), "c".to_string()],
            second_compartment: vec!["D".to_string(), "E".to_string(), "f".to_string()],
        };
        assert_eq!(parse_rucksack(&"aBcDEf".to_string()), expected);
    }

    #[test]
    fn test_find_duplicates() {
        let rucksack = Rucksack {
            first_compartment: vec![
                "a".to_string(),
                "B".to_string(),
                "f".to_string(),
                "c".to_string(),
                "f".to_string(),
            ],
            second_compartment: vec![
                "A".to_string(),
                "B".to_string(),
                "d".to_string(),
                "f".to_string(),
            ],
        };

        let expected = HashSet::from(["B".to_string(), "f".to_string()]);
        assert_eq!(find_duplicates(&rucksack), expected)
    }

    #[test]
    fn test_calculate_priorities() {
        let duplicates = vec![
            "p".to_string(),
            "L".to_string(),
            "P".to_string(),
            "v".to_string(),
            "t".to_string(),
            "s".to_string(),
        ];
        let expected = vec![16, 38, 42, 22, 20, 19];

        assert_eq!(calculate_priorities(duplicates), expected)
    }

    #[test]
    fn test_create_groups() {
        let lines: Vec<String> = vec![
            "ab".to_string(),
            "cd".to_string(),
            "ef".to_string(),
            "gh".to_string(),
            "ij".to_string(),
            "kl".to_string(),
        ];
        let expected = vec![
            Group {
                rucksacks: vec![
                    Rucksack {
                        first_compartment: vec!["a".to_string()],
                        second_compartment: vec!["b".to_string()],
                    },
                    Rucksack {
                        first_compartment: vec!["c".to_string()],
                        second_compartment: vec!["d".to_string()],
                    },
                    Rucksack {
                        first_compartment: vec!["e".to_string()],
                        second_compartment: vec!["f".to_string()],
                    },
                ],
            },
            Group {
                rucksacks: vec![
                    Rucksack {
                        first_compartment: vec!["g".to_string()],
                        second_compartment: vec!["h".to_string()],
                    },
                    Rucksack {
                        first_compartment: vec!["i".to_string()],
                        second_compartment: vec!["j".to_string()],
                    },
                    Rucksack {
                        first_compartment: vec!["k".to_string()],
                        second_compartment: vec!["l".to_string()],
                    },
                ],
            },
        ];
        assert_eq!(create_groups(lines), expected);
    }

    #[test]
    fn test_find_badge_item_type() {
        let groups = create_groups(vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ]);

        let expected = HashSet::from(["r".to_string(), "Z".to_string()]);
        assert_eq!(find_badge_item_types(groups), expected)
    }

    #[test]
    fn test_calculate_priorities_for_badge_item_types() {
        let badge_item_types = HashSet::from(["r".to_string(), "Z".to_string()]);
        
        let expected : Vec<i32> = vec![18, 52];
        assert_eq!(
            calculate_priorities(Vec::from_iter(badge_item_types.iter().cloned())), 
            expected
        )
    }
}
