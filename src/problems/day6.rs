use super::common::read_file;
use std::collections::HashSet;

pub fn part1() -> usize {
    let l = read_file("test_files/6.txt").chars().collect::<Vec<_>>();
    for i in 3..l.len() {
        let slice = l.iter().skip(i - 3).take(4).map(|c| c.clone());
        let hashset = HashSet::<char>::from_iter(slice);
        if hashset.len() == 4 {
            return i + 1;
        }
    }
    0
}

pub fn part2() -> usize {
    let l = read_file("test_files/6.txt").chars().collect::<Vec<_>>();
    for i in 13..l.len() {
        let slice = l.iter().skip(i - 13).take(14).map(|c| c.clone());
        let hashset = HashSet::<char>::from_iter(slice);
        if hashset.len() == 14 {
            return i + 1;
        }
    }
    0
}
