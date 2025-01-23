pub fn parse(input: &str) -> Vec<String> {
    input.split('\n')
         .filter(|s| !s.is_empty())
         .map(|s| s.to_string())
         .collect()
}

pub fn part1(input: &[String]) -> usize {
    let mut total: usize = 0;
    for line in input {
        total += line.len();

        let mut sz = line.len();
        let mut ind: usize = 0;
        while ind < line.len() {
            sz -= match line.chars().nth(ind) {
                Some('\"') => 1,
                Some('\\') => match line.chars().nth(ind+1) {
                    Some('\"' | '\\') => 1,
                    Some('x') => 3,
                    _ => unreachable!(),
                },
                Some(_) => 0,
                None => unreachable!(),
            };
            ind += match line.chars().nth(ind) {
                Some('\\') => {
                    match line.chars().nth(ind+1) {
                        Some('\"' | '\\') => 2,
                        Some('x') => 4,
                        _ => unreachable!(),
                    }
                },
                _ => 1,
            };
        }
        total -= sz;
    }

    total
}

pub fn part2(input: &[String]) -> usize {
    let mut total: usize = 0;
    for line in input {
        for c in line.chars() {
            total += match c {
                '\"'|'\\' => 2,
                _ => 1,
            };
        }
        total += 2;
        total -= line.len();
    }

    total
}
