use super::common::read_file;

pub fn part1() -> u32 {
    read_file("test_files/2.txt")
        .lines()
        .map(|x| {
            let mut split = x.split(" ");
            let theirs = split.next().unwrap();
            let yours = split.next().unwrap();

            let vicory = match (theirs, yours) {
                ("A", "Y") => 6,
                ("B", "Z") => 6,
                ("C", "X") => 6,
                ("A", "Z") => 0,
                ("B", "X") => 0,
                ("C", "Y") => 0,
                (_, _) => 3,
            };
            let chosen = match yours {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => panic!("Not a possibility"),
            };
            vicory + chosen
        })
        .sum()
}

pub fn part2() -> u32 {
    read_file("test_files/2.txt")
        .lines()
        .map(|x| {
            let mut split = x.split(" ");
            let theirs = split.next().unwrap();
            let action = split.next().unwrap();

            let theirs = match theirs {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => panic!("Not a possibility"),
            };

            match (action, theirs) {
                ("X", 1) => 0 + 3,
                ("X", _) => 0 + theirs - 1,
                ("Y", _) => 3 + theirs,
                ("Z", _) => 6 + (theirs % 3) + 1,
                _ => panic!("Not a possibility"),
            }
        })
        .sum()
}
