pub fn parse(input: &str) -> Vec<String> {
    input.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect()
}

pub fn has_vowels(s: &str) -> bool {
    let mut total: i32 = 0;
    for c in s.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                total += 1;
                if total == 3 {
                    return true;
                }
            },
            _ => (),
        };
    }
    false
}

pub fn has_double(s: &str) -> bool {
    for i in 0..s.len()-1 {
        if s.chars().nth(i).unwrap() == s.chars().nth(i+1).unwrap() {
            return true;
        }
    }
    false
}

pub fn has_forbidden(s: &str) -> bool {
    for i in 0..s.len()-1 {
        match s.chars().nth(i).unwrap() {
            'a' => {
                if s.chars().nth(i+1).unwrap() == 'b' {
                    return true;
                }
            },
            'c' => {
                if s.chars().nth(i+1).unwrap() == 'd' {
                    return true;
                }
            },
            'p' => {
                if s.chars().nth(i+1).unwrap() == 'q' {
                    return true;
                }
            },
            'x' => {
                if s.chars().nth(i+1).unwrap() == 'y' {
                    return true;
                }
            },
            _ => (),
        }
    }
    false
}

pub fn has_sandwhich(s: &str) -> bool {
    for i in 0..s.len()-2 {
        if s.chars().nth(i).unwrap() == s.chars().nth(i+2).unwrap() {
            return true;
        }
    }
    false
}

pub fn has_double_pair(s: &str) -> bool {
    for i in 0..s.len()-1 {
        for j in i+2..s.len()-1 {
            if s.chars().nth(i).unwrap() == s.chars().nth(j).unwrap() && s.chars().nth(i+1).unwrap() == s.chars().nth(j+1).unwrap() {
                return true;
            }
        }
    }

    false
}

pub fn part1(input: &[String]) -> i32 {
    let mut total: i32 = 0;
    for s in input {
        if has_vowels(s) && has_double(s) && !has_forbidden(s) {
            total += 1;
        }
    }
    total
}

pub fn part2(input: &[String]) -> i32 {
    let mut total: i32 = 0;
    for s in input {
        if has_sandwhich(s) && has_double_pair(s) {
            total += 1;
        }
    }
    total
}
