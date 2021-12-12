
pub fn part_a(input: &str) -> i64 {
    0
}

pub fn part_b(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a_test() {
        assert_eq!(super::part_a(include_str!("testdata.txt")), 0);
    }
    #[test]
    fn part_a_real() {
        assert_eq!(super::part_a(include_str!("realdata.txt")), 0);
    }

    #[test]
    fn part_b_test() {
        assert_eq!(super::part_b(include_str!("testdata.txt")), 0);
    }

    #[test]
    fn part_b_real() {
        assert_eq!(super::part_b(include_str!("realdata.txt")), 0);
    }
}
