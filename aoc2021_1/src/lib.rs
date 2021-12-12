
pub fn part_a(input: &str) -> i64 {
    let c: Vec<i64> = input
        .trim()
        .split('\n')
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    let mut increases = 0;
    for window in c.windows(2) {  // get a window of the first 2 entries
        if window[1] > window[0] {
            increases += 1;
        }
    }
    increases
}

pub fn part_b(input: &str) -> i64 {
    let c: Vec<i64> = input
        .trim()
        .split('\n')
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    let mut increases = 0;
    for window in c.windows(4) {  // get a window of the first 2 entries
        if window[1] + window[2] + window[3] > window[0] + window[1] + window[2] {
            increases += 1;
        }
    }
    increases
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a_test() {
        assert_eq!(super::part_a(include_str!("testdata.txt")), 7);
    }
    #[test]
    fn part_a_real() {
        assert_eq!(super::part_a(include_str!("realdata.txt")), 1400);
    }

    #[test]
    fn part_b_test() {
        assert_eq!(super::part_b(include_str!("testdata.txt")), 5);
    }

    #[test]
    fn part_b_real() {
        assert_eq!(super::part_b(include_str!("realdata.txt")), 1429);
    }
}
