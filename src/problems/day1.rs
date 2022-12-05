use std::fs::File;
use std::io::{read_to_string, BufReader};

pub fn part1() -> u32 {
    let text = read_to_string(BufReader::new(
        File::open("test_files/1.txt").expect("day file missing"),
    ))
    .expect("Couldn't read");

    let mut groups = Vec::new();
    let mut current_group = Vec::new();
    for line in text.lines() {
        if line != "" {
            current_group.push(line.parse::<u32>().unwrap());
        } else {
            groups.push(current_group);
            current_group = Vec::new();
        }
    }
    return groups.iter().map(|g| g.iter().sum()).max().unwrap();
}

pub fn part2() -> u32 {
    let text = read_to_string(BufReader::new(
        File::open("test_files/1.txt").expect("day file missing"),
    ))
    .expect("Couldn't read");

    let mut groups = Vec::new();
    let mut current_group = Vec::new();
    for line in text.lines() {
        if line != "" {
            current_group.push(line.parse::<u32>().unwrap());
        } else {
            groups.push(current_group);
            current_group = Vec::new();
        }
    }
    let mut groups = groups.iter().map(|g| g.iter().sum()).collect::<Vec<_>>();
    groups.sort();
    return groups.iter().rev().take(3).sum();
}
