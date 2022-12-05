use super::common::read_file;
use std::collections::HashSet;

pub fn part1() -> u32 {
    read_file("test_files/3.txt")
        .lines()
        .map(|l| {
            let first = &l[0..l.len() / 2];
            let second = &l[l.len() / 2..];
            for c in second.chars() {
                if first.contains(c) {
                   return if ('a'..='z').contains(&c) {
                        (c as u32) - 96
                    } else {
                        (c as u32) - 38
                    };
                }
            }
            return 0;
        })
        .sum()
}

pub fn part2() -> u32 {
    read_file("test_files/3.txt")
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|x| {
            let sec = x[1].chars().collect::<HashSet<_>>();
            let third = x[2].chars().collect::<HashSet<_>>();
            x[0].chars()
                .collect::<HashSet<_>>()
                .iter()
                .filter(|&c| sec.contains(c) && third.contains(c))
                .next()
                .map(|&c| {
                    if ('a'..='z').contains(&c) {
                        (c as u32) - 96
                    } else {
                        (c as u32) - 38
                    }
                }).unwrap()
        })
        .sum()
}
