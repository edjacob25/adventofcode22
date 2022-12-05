use super::common::read_file;
use std::collections::VecDeque;

fn parse_initial_state(lines: Vec<&str>) -> Vec<Vec<char>> {
    let mut initial_it = lines.iter().rev();
    let number_of_stacks = initial_it
        .next()
        .unwrap()
        .split(" ")
        .filter(|&c| c != "")
        .count();

    let mut stacks = Vec::new();

    for _ in 0..number_of_stacks {
        stacks.push(Vec::new())
    }
    for l in initial_it {
        let mut whitesspace_counter = 0;
        let mut idx = 0;
        for c in l.chars().skip(1) {
            if whitesspace_counter > 3 {
                whitesspace_counter = 0;
                idx += 1;
            }

            if c.is_whitespace() {
                whitesspace_counter += 1;
                continue;
            }

            if ('A'..='Z').contains(&c) {
                stacks.get_mut(idx).unwrap().push(c);
                idx += 1;
                whitesspace_counter = 0;
            }
        }
    }
    stacks
}

pub fn part1() -> String {
    let mut initial_part = Vec::new();
    let lines = read_file("test_files/5.txt");
    let it = lines.lines();
    for l in it {
        if l == "" {
            break;
        }
        initial_part.push(l);
    }
    let mut stacks = parse_initial_state(initial_part);
    let it = lines.lines();
    let mut second_part = false;
    for l in it {
        if l == "" {
            second_part = true;
            continue;
        }
        if !second_part {
            continue;
        }

        let split = l.split(" ").collect::<Vec<_>>();
        let quantity = split[1].parse::<u32>().unwrap();
        let source = split[3].parse::<usize>().unwrap() - 1;
        let destination = split[5].parse::<usize>().unwrap() - 1;
        let source = stacks.get_mut(source).unwrap();

        let mut moving = Vec::new();
        for _ in 0..quantity {
            // println!("{}", l);
            // println!("{:?} {}", source, quantity);
            moving.push(source.pop().unwrap());
        }
        assert_eq!(quantity as usize, moving.len());
        let destination = stacks.get_mut(destination).unwrap();
        destination.append(&mut moving);
    }
    stacks.iter().map(|v| v.last().unwrap()).collect()
}

pub fn part2() -> String {
    let mut initial_part = Vec::new();
    let lines = read_file("test_files/5.txt");
    let it = lines.lines();
    for l in it {
        if l == "" {
            break;
        }
        initial_part.push(l);
    }
    let mut stacks = parse_initial_state(initial_part);
    let it = lines.lines();
    let mut second_part = false;
    for l in it {
        if l == "" {
            second_part = true;
            continue;
        }
        if !second_part {
            continue;
        }

        let split = l.split(" ").collect::<Vec<_>>();
        let quantity = split[1].parse::<u32>().unwrap();
        let source = split[3].parse::<usize>().unwrap() - 1;
        let destination = split[5].parse::<usize>().unwrap() - 1;
        let source = stacks.get_mut(source).unwrap();

        let mut moving = VecDeque::new();
        for _ in 0..quantity {
            moving.push_front(source.pop().unwrap());
        }
        assert_eq!(quantity as usize, moving.len());
        let mut moving = moving.into();
        let destination = stacks.get_mut(destination).unwrap();
        destination.append(&mut moving);
    }
    stacks.iter().map(|v| v.last().unwrap()).collect()
}
