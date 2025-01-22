pub fn parse(input: &str) -> Vec<i32> {
    let mut parsed = Vec::<i32>::new();
    for c in input.chars() {
        match c {
            '(' => parsed.push(1),
            ')' => parsed.push(-1),
            '\n' => (),
            _ => panic!(),
        }
    }

    parsed
}

pub fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

pub fn part2(input: &[i32]) -> usize {
    let mut total: i32 = 0;
    for (i, step) in input.iter().enumerate() {
        total += step;
        if total == -1 {
            return i + 1;
        }
    }

    unreachable!()
}
