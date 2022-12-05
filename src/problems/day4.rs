use super::common::read_file;

fn convert_into_pairs(line: &str) -> Vec<(u32, u32)> {
    let mut pairs = Vec::new();
    for p in line.split(",") {
        let mut sp = p.split("-");
        let initial = sp.next().unwrap().parse::<u32>().unwrap();
        let fina = sp.next().unwrap().parse::<u32>().unwrap();
        pairs.push((initial, fina));
    }
    pairs
}

pub fn part1() -> u32 {
    read_file("test_files/4.txt")
        .lines()
        .map(|l| {
            let pairs = convert_into_pairs(l);
            return if (pairs[0].0 >= pairs[1].0 && pairs[0].1 <= pairs[1].1)
                || (pairs[1].0 >= pairs[0].0 && pairs[1].1 <= pairs[0].1)
            {
                1
            } else {
                0
            };
        })
        .sum()
}

pub fn part2() -> u32 {
    read_file("test_files/4.txt")
        .lines()
        .map(|l| {
            let pairs = convert_into_pairs(l);
            return if (pairs[0].0 >= pairs[1].0 && pairs[0].1 <= pairs[1].1)
                || (pairs[1].0 >= pairs[0].0 && pairs[1].1 <= pairs[0].1)
                || (pairs[0].0 >= pairs[1].0 && pairs[0].0 <= pairs[1].1)
                || (pairs[0].1 >= pairs[1].0 && pairs[0].1 <= pairs[1].1)
            {
                1
            } else {
                0
            };
        })
        .sum()
}
