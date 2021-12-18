/*

 */

pub fn part_a(input: &str) -> i64 {
    let lines: Vec<_> = input.trim().split('\n').collect();
    let line_count = lines.len();
    let column_count = lines[0].len();
    let mut ones = vec![0usize; column_count];

    for line in &lines {
//        println!("line is {}", line);
        for (index, character) in line.chars().enumerate() {
            if character == '1' {
                ones[index] += 1;
            }
        };
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();
    // need to loop through the ones array and create the strings
    for i in 0..column_count {
        println!("Running for column: {} with column count {}", i, column_count);
        let ones = ones[i];
        println!("ones {}", ones);
        let zeros = line_count - ones;
        println!("ones {} zeros {}", ones, zeros);
        if ones > zeros {
            println!("Gamma triggered");
            gamma.push('1');
            epsilon.push('0');
        } else {
            println!("Epsilon triggered");
            gamma.push('0');
            epsilon.push('1');
        }
    }
    let gamma_int = i64::from_str_radix(&gamma, 2).unwrap();
    let epsilon_int = i64::from_str_radix(&epsilon, 2).unwrap();
    println!("gamma_int {}", gamma_int);
    println!("epsilon_int {}", epsilon_int);

    gamma_int * epsilon_int
}

pub fn part_b(input: &str, mostCommon: bool) -> i64 {
    let mut lines: Vec<_> = input.trim().split('\n').collect();
    let column_count = lines[0].len();

    let mut derived_code_oxygen = String::new();

    // loop though and find the most common 1 or 0 in position n starting at 0
    for i in 0..column_count {
        println!("processing column {}", i);
        let mut ones = 0;
        let mut zeros = 0;
        for d in 0usize..lines.len() {
            println!("line is {}", lines[d]);
                println!("checking value {}", lines[d].chars().nth(i).unwrap());
                if lines[d].chars().nth(i).unwrap() == '1' {
                    ones += 1;
                } else {
                    zeros += 1;
                }
                println!("ones: {}  zeros: {}", ones, zeros);
            // }
        }
        if mostCommon {
            if ones >= zeros {
                derived_code_oxygen.push('0');
            } else {
                derived_code_oxygen.push('1');
            }
        } else {
            if ones >= zeros {
                derived_code_oxygen.push('1');
            } else {
                derived_code_oxygen.push('0');
            }
        }

        lines = lines.into_iter()
            .filter(|x| x.starts_with(&derived_code_oxygen))
            .collect();

        println!("derived_code_oxygen so far is {}", derived_code_oxygen);
        if lines.len() == 1 { return i64::from_str_radix(lines[0], 2).unwrap()}
    }

    0
}


#[cfg(test)]
mod tests {
    #[test]
    fn part_a_test() {
        assert_eq!(super::part_a(include_str!("testdata.txt")), -1);
    }

    #[test]
    fn part_a_real() {
        assert_eq!(super::part_a(include_str!("realdata.txt")), -1);
    }

    #[test]
    fn part_b_test() {
        assert_eq!(super::part_b(include_str!("testdata.txt"), true)
                   * super::part_b(include_str!("testdata.txt"), false)
                   , -1);
    }

    #[test]
    fn part_b_real() {
        assert_eq!(super::part_b(include_str!("realdata.txt"), true)
                       * super::part_b(include_str!("realdata.txt"), false)
                   , -1);
    }
}
