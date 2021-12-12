
pub fn part_a(input: &str) -> i64 {
    let mut horizontal = 0;
    let mut depth = 0;
    
    for line in input.trim().split('\n') {
        let mut parts = line.split(' ');
        let direction = parts.next().unwrap();
        let amount: i64 = parts.next().unwrap().parse().unwrap();
        
        match direction {
            "forward" => { horizontal += amount }
            "down"    => { depth += amount }
            "up"      => { depth -= amount }
            _ => panic!()
        }
    }
    return horizontal * depth
}

pub fn part_b(input: &str) -> i64 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    
    for line in input.trim().split('\n') {
        let mut parts = line.split(' ');
        let direction = parts.next().unwrap();
        let amount: i64 = parts.next().unwrap().parse().unwrap();
        
        match direction {
            "forward" => { 
                horizontal += amount;
                depth += aim * amount;
            }
            "down"    => { aim += amount }
            "up"      => { aim -= amount }
            _ => panic!()
        }
    }
    return horizontal * depth
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a_test() {
        assert_eq!(super::part_a(include_str!("testdata.txt")), 150);
    }
    #[test]
    fn part_a_real() {
        assert_eq!(super::part_a(include_str!("realdata.txt")), 1524750);
    }

    #[test]
    fn part_b_test() {
        assert_eq!(super::part_b(include_str!("testdata.txt")), 900);
    }

    #[test]
    fn part_b_real() {
        assert_eq!(super::part_b(include_str!("realdata.txt")), 1592426537);
    }
}
