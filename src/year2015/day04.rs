use md5;

pub fn parse(input: &str) -> &str {
    input.strip_suffix("\n").unwrap_or(input)
}

pub fn leading_zeros(hash: md5::Digest) ->  usize{
    for i in 0..16 {
        if hash.0[i] / 16 > 0 {
            return 2 * i;
        }

        if hash.0[i] % 16 > 0 {
            return 2 * i + 1;
        }
    }
    unreachable!();
}

pub fn part1(input: &str) -> i32 {
    let mut i: i32 = 0;
    loop {
        if leading_zeros(md5::compute(format!("{input}{i}"))) == 5 {
            return i;
        }
        i += 1;
    }
}

pub fn part2(input: &str) -> i32 {
    let mut i: i32 = 0;
    loop {
        if leading_zeros(md5::compute(format!("{input}{i}"))) == 6 {
            return i;
        }
        i += 1;
    }
}
