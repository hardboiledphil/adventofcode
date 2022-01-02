/*
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
 */

// #[derive(Copy, Clone)]
pub struct BoardNumber {
    is_called: bool,
    y: i32,
    x: i32,
    value: i32,
}

impl BoardNumber {
    fn new(is_called: bool, y: i32, x: i32, value: i32) -> BoardNumber {
        return BoardNumber { is_called, y, x, value };
    }
}

pub struct Board<'a> {
    board_name: String,
    bingo_numbers: &'a mut Vec<BoardNumber>,
    bingoed_out: bool,
}

impl Board<'_> {
    fn isCalled(&self, position_to_check: (i32, i32)) -> bool {
        println!("  checking y, x: {} {}", position_to_check.0, position_to_check.1);
        // let matching_numbers = false;
        for board_number_to_check in 0..25 {
            // let board_number_to_check_val = self.bingo_numbers[board_number_to_check];
            if self.bingo_numbers[board_number_to_check].y == position_to_check.0
                && self.bingo_numbers[board_number_to_check].x == position_to_check.1
                && self.bingo_numbers[board_number_to_check].is_called {
                return true
            }
        }
        return false
    }
}

pub trait Bingo {
    fn accept_call(&mut self, call: i32) -> bool;
    fn is_bingo(&mut self) -> bool;
    fn sum_uncalled(&self) -> i32;
}

impl Bingo for Board<'_> {
    // process the call for the board
    fn accept_call(&mut self, call: i32) -> bool {
        // loop through the board and mark as called if call number is found
        let mut myvec = self.bingo_numbers.iter_mut();

        for board_number in myvec {
            if board_number.value == call {
                board_number.is_called = true;
                return true;
            }
        }
        println!("  marking board {} for call {}", self.board_name, call);
        return false;
    }

    fn is_bingo(&mut self) -> bool {
        // let bingo_numbers = self.boardNumbers.as_slice();
        // check if the board has full rows
        for y in 0..5 {
            if self.isCalled((y, 0))
                && self.isCalled((y, 1))
                && self.isCalled((y, 2))
                && self.isCalled((y, 3))
                && self.isCalled((y, 4)) {
                println!("    BINGO for board {}", self.board_name);
                self.bingoed_out = true;
                return true
            }
        }

        // check if the board has full columns
        for x in 0..5 {
            if self.isCalled((0, x))
                && self.isCalled((1, x))
                && self.isCalled((2, x))
                && self.isCalled((3, x))
                && self.isCalled((4, x)) {
                self.bingoed_out = true;
                println!("    BINGO for board {}", self.board_name);
                return true
            }
        }
        false
    }

    fn sum_uncalled(&self) -> i32 {
        let mut sum_total_uncalled = 0;
        for x in 0..25 {
           let test_board_number = self.bingo_numbers.get(x).unwrap();
            if !test_board_number.is_called {
                sum_total_uncalled += test_board_number.value;
            }
            // match self.bingo_numbers.get(x).unwrap())
        }
        sum_total_uncalled
    }
}

pub fn process_calls<'a, 'b>(call_value: i32, boards: &'a mut Vec<Board>) -> (i32, Option<String>) {
    // let mut bingo_board = (0, None);
    // boards.
    let y = boards.iter_mut();
    // let mut z = y.filter(|boxed_boardV| !(boxed_board).bingoed_out ).collect();
    for board in y {
        println!("calling number {}", call_value);
        let matches_call = board.accept_call(call_value);
        // we got a match so see if we have a winner
        if matches_call {
            if board.is_bingo() {
                // println!("We have a winner on board {}", board.board_name);
                return (call_value, Some(board.board_name));
            }
        }
    }
    return (0, None)
}

pub fn part_a(input: &str) -> i64 {
    let mut lines = input.trim().split('\n');

    // read the line for the numbers
    let calls: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|number_str| number_str.parse().unwrap())
        .collect();

    let mut boards: &mut Vec<Board> = &mut Vec::new();

    let mut board_count = 0;
    // read the rest of the lines and generate the boards
    while let Some(_) = lines.next() {
        // let mut line_count :i32 = 0;
        let mut bingo_numbers: &Vec<BoardNumber> = &vec![];
        // let board_numbers_ref = &bingo_numbers;
        for y in 0..5 {
            println!("starting board number {}", board_count);
            for (x, number_as_str) in lines.next().unwrap().split(' ').enumerate() {
//                println!("line {} column {} number {}", y, x, number_as_str);
                if !number_as_str.is_empty() {
                    let number = number_as_str.parse().unwrap();
                    let board_number = BoardNumber::new(false, y, x as i32, number);
                    // let board_number_position = ((y * 5) + x as i32) as usize;
                    bingo_numbers.push(board_number);
                }
                // line_count += 1;
            }
        }
//        println!("data lines when flattened is {}", data_lines);
        if !bingo_numbers.is_empty() {
            let new_board = Board { bingo_numbers: bingo_numbers,
                board_name: board_count.to_string(),
                bingoed_out: false };
            boards.push(new_board);
            board_count += 1;
        }
    }

    // feed the numbers into the boards and get them to check for a row/column
    let mut winning_board = (0, None);
    for call_value in calls.as_slice().into_iter() {
        winning_board = process_calls(*call_value, boards);
        if winning_board.1.is_some() {
            break;
        }
    }

    match winning_board {
        // (0, _) means that no board was returned
        (0, _) => return 0,
        // This match means we got something - so add up all the unmatched
        // and multiply by the last number called
        (_, Some(_)) => {
            let name = winning_board.1.unwrap();
            let sum_not_called = boards
                .iter()
                .filter(|x| x.board_name == name)
                .map(|board| board.sum_uncalled() as i64)
                .sum::<i64>() as i64;
            return sum_not_called * winning_board.0 as i64
        },
        (_, _) => panic!("Shouldn't end up here"),
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn part_a_test() {
        assert_eq!(super::part_a(include_str!("testdata.txt")), 4512);
    }

    #[test]
    fn part_a_real() {
        assert_eq!(super::part_a(include_str!("realdata.txt")), 11774);
    }

    // #[test]
    // fn part_b_test() {
    //     assert_eq!(super::part_b(include_str!("testdata.txt")), 1924);
    // }
    //
    // #[test]
    // fn part_b_real() {
    //     assert_eq!(super::part_b(include_str!("realdata.txt")), -1);
    // }
}
