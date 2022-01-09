use std::ops::{Div, Rem};

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let rem = dividend.rem(divisor);
    let div = dividend.div(divisor);
    (div, rem)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
   iter.enumerate().filter(|(i, item)| i.rem(2) == 0 ).map(|(i, item)| item)
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        self.0.abs() + self.1.abs()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_manhattan_positive() {
        let my_pos = super::Position { 0: 3, 1: 7 };
        let man_value = my_pos.manhattan();
        assert_eq!(man_value, 10);
    }

    #[test]
    fn test_manhattan_one_negative() {
        let my_pos = super::Position { 0: -3, 1: 7 };
        let man_value = my_pos.manhattan();
        assert_eq!(man_value, 10);
    }

    #[test]
    fn test_one() {
        assert_eq!(super::divmod(10, 3), (3, 1));
    }

    #[test]
    fn test_evens() {
        let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let exp_result = vec![0, 2, 4, 6, 8, 10];
        // assert_eq!(my_vec, exp_result);
        let new_vec: Vec<i32> = super::evens(my_vec.into_iter()).collect();
        assert_eq!(new_vec, exp_result);
    }
}
