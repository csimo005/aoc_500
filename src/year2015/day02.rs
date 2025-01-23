use std::cmp;

pub fn parse(input: &str) -> Vec<i32> {
    let mut data = Vec::<i32>::new();
    for line in input.lines() {
        for f in line.split('x') {
            data.push(f.parse::<i32>().unwrap());
        }
    }

    data
}

pub fn part1(input: &[i32]) -> i32 {
    let mut total: i32 = 0;
    for i in (0..input.len()).step_by(3) {
        let (l, w, h) = (input[i], input[i+1], input[i+2]);
        total += 2 * (l * w + w * h + l * h) + (l * w * h) / cmp::max(cmp::max(l, w), h);
    }

    total
}

pub fn part2(input: &[i32]) -> i32 {
    let mut total: i32 = 0;
    for i in (0..input.len()).step_by(3) {
        let (l, w, h) = (input[i], input[i+1], input[i+2]);
        total += 2 * (l + w + h - cmp::max(cmp::max(l, w), h));
        total += l * w * h;
    }

    total
}
