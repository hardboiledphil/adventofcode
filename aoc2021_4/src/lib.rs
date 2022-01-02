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

#[derive(Debug)]
pub struct Position {
    y: i32,
    x: i32,
}

impl Position {
    fn new(y: i32, x: i32) -> Position {
        return Position { y, x };
    }
}

#[derive(Debug)]
pub struct BoardNumber {
    is_called: bool,
    location: Position,
    value: i32,
}

impl BoardNumber {
    fn new(is_called: bool, position: Position, value: i32) -> BoardNumber {
        return BoardNumber { is_called, location: position, value };
    }
}

#[derive(Debug)]
pub struct Board {
    board_name: String,
    board_numbers: Vec<BoardNumber>,
    bingoed_out: bool,
}

impl Board {
    fn is_called(&self, y: i32, x: i32) -> bool {
        // println!("  checking y, x: {}, {}", y, x);
        let matching_numbers =
            self.board_numbers.as_slice()
                .into_iter()
                .filter(|board_number|
                    y == board_number.location.y
                    && x == board_number.location.x
                    && true == board_number.is_called )
                .map(|input| {
//                    println!("  found boardname: {} y: {} x: {}, value: {}", self.boardName, y, x, input.value);
                    return input; })
                .next();
        if matching_numbers.is_some() {
//           println!("  sending back called = true");
            true
        } else {
//            println!("  sending back called = false");
            false
        }
    }
}

pub trait Bingo {
    fn accept_call(&mut self, call: i32) -> bool;
    fn is_bingo(&mut self) -> bool;
    fn sum_uncalled(&self) -> i32;
}

impl Bingo for Board {
    // process the call for the board
    fn accept_call(&mut self, call: i32) -> bool {
        // loop through the board and mark as called if call number is found
        let myvec = self.board_numbers.as_mut_slice();

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
        // let board_numbers = self.boardNumbers.as_slice();
        // check if the board has full rows
        for y in 0..5 {
            if self.is_called(y, 0)
                && self.is_called(y, 1)
                && self.is_called(y, 2)
                && self.is_called(y, 3)
                && self.is_called(y, 4) {
                println!("    BINGO for board {}", self.board_name);
                self.bingoed_out = true;
                return true
            }
        }

        // check if the board has full columns
        for x in 0..5 {
            if self.is_called(0, x)
                && self.is_called(1, x)
                && self.is_called(2, x)
                && self.is_called(3, x)
                && self.is_called(4, x) {
                self.bingoed_out = true;
                println!("    BINGO for board {}", self.board_name);
                return true
            }
        }
        false
    }

    fn sum_uncalled(&self) -> i32 {
        self.board_numbers
            .as_slice()
            .into_iter()
            .filter( |board_number| board_number.is_called == false)
            .map( |board_number| board_number.value)
            .sum()
    }
}

pub fn process_calls(call_value: i32, boards: &mut Vec<Board>) -> (i32, Option<&mut Board>) {
    let mut bingo_board = (0, None);

    let new_boards =
        boards.as_mut_slice().iter_mut().filter(|board| !board.bingoed_out);
    for board in new_boards {
        println!("calling number {}", call_value);
        let matches_call = board.accept_call(call_value);
        // we got a match so see if we have a winner
        if matches_call {
            if board.is_bingo() {
                println!("We have a winner on board {}", board.board_name);
                bingo_board = (call_value, Some(board));
                return bingo_board
            }
        }
    }
    return bingo_board
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

    let boards: &mut Vec<Board> = &mut Vec::new();

    let mut board_count = 0;
    // read the rest of the lines and generate the boards
    while let Some(_) = lines.next() {
        let mut board_numbers: Vec<BoardNumber> = Vec::new();
        for y in 0..5 {
            println!("starting board number {}", board_count);
            for (x, number_as_str) in lines.next().unwrap().split(' ').enumerate() {
               println!("line {} column {} number {}", y, x, number_as_str);
                if !number_as_str.is_empty() {
                    let number = number_as_str.parse().unwrap();
                    let board_number = BoardNumber::new(false, Position::new(y, x as i32), number);
                    board_numbers.push(board_number);
                }
            }
        }
       // println!("data lines when flattened is {}", data_lines);
        if !board_numbers.is_empty() {
            let new_board = Board { board_numbers: board_numbers,
                board_name: board_count.to_string(),
                bingoed_out: false };
            boards.push(new_board);
            board_count += 1;
        }
    }

    // feed the numbers into the boards and get them to check for a row/column
    let mut winning_board: (i32, Option<&mut Board>) = (0, None);
    for call_value in calls.into_iter() {
        winning_board = process_calls(call_value, boards);
        if winning_board.1.is_some() {
            break;
        }
    }

    match winning_board {
        (0, _) => return 0,
        (_, Some(_)) => return winning_board.1.unwrap()
            .board_numbers
            .as_slice().into_iter()
            .filter(|board_number| board_number.is_called == false)
            .map(|board| board.value as i64)
            .sum::<i64>() * winning_board.0 as i64,
        (_, _) => panic!("Shouldn't end up here"),
    }

}

pub fn part_b(input: &str) -> i64 {
    let mut lines = input.trim().split('\n');

    // read the line for the numbers
    let calls: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|number_str| number_str.parse().unwrap())
        .collect();

    let boards: &mut Vec<Board> = &mut Vec::new();

    let mut board_count = 0;
    // read the rest of the lines and generate the boards
    while let Some(_) = lines.next() {
        let mut board_numbers: Vec<BoardNumber> = Vec::new();
        for y in 0..5 {
            println!("starting board number {}", board_count);
            for (x, number_as_str) in lines.next().unwrap().split(' ').enumerate() {
//                println!("line {} column {} number {}", y, x, number_as_str);
                if !number_as_str.is_empty() {
                    let number = number_as_str.parse().unwrap();
                    let board_number = BoardNumber::new(false, Position::new(y, x as i32), number);
                    board_numbers.push(board_number);
                }
            }
        }
//        println!("data lines when flattened is {}", data_lines);
        if !board_numbers.is_empty() {
            let new_board = Board { board_numbers: board_numbers,
                board_name: board_count.to_string(),
                bingoed_out: false};
            boards.push(new_board);
            board_count += 1;
        }
    }

    // feed the numbers into the boards and get them to check for a row/column
    let mut winning_boards: Vec<(i32, Option<&mut Board>)> = Vec::new();
    // let mut winning_board: (i32, Option<&mut Board>) = (0, None);
    for call_value in calls.into_iter() {
        winning_boards.push(process_calls(call_value, boards));
    }

    println!("winning board is {:?}", winning_boards);

    // match winning_board {
    //     (_, None) => return 0,
    //     (_, Some(_)) => return winning_board.1.unwrap()
    //         .board_numbers
    //         .as_slice().into_iter()
    //         .filter(|board_number| board_number.is_called == false)
    //         .map(|board| board.value as i64)
    //         .sum::<i64>() * winning_board.0 as i64,
    // }
    0
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

    #[test]
    fn part_b_test() {
        assert_eq!(super::part_b(include_str!("testdata.txt")), 1924);
    }

    #[test]
    fn part_b_real() {
        assert_eq!(super::part_b(include_str!("realdata.txt")), -1);
    }
}
