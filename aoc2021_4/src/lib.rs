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

pub struct Position {
    y: i32,
    x: i32,
}

impl Position {
    fn new(y: i32, x: i32) -> Position {
        return Position { y, x };
    }
}

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

pub struct Board {
    boardName: String,
    boardNumbers: Vec<BoardNumber>,
}

impl Board {
    fn isCalled(&self, y: i32, x: i32) -> bool {
        println!("checking y, x: {}, {}", y, x);
        let exists =
            self.boardNumbers.as_slice()
                .into_iter()
                .map(|input| {
                    println!("boardname: {} y: {} x: {}", self.boardName, y, x);
                    return input; })
                .filter(|board_number| y == board_number.location.y)
                .filter(|board_number| x == board_number.location.x)
                .filter(|board_number| true == board_number.is_called)
                .next();
        if Some(exists).unwrap_or_else(|| true == false ) {
            return true;
        } else {
            return false;
        }
    }
}

pub trait Bingo {
    fn accept_call(&mut self, call: i32) -> bool;
    fn is_bingo(&self) -> bool;
}

impl Bingo for Board {
    // process the call for the board
    fn accept_call(&mut self, call: i32) -> bool {
        println!("board {} accepting call {}", self.boardName, call);
        // loop through the board and mark as called if call number is found
        let myvec = self.boardNumbers.as_mut_slice();

        for board_number in myvec {
//            println!("boardnumber {} call {}", board_number.value, call);
            if board_number.value == call {
                println!("matched on boardnumber {}", self.boardName);
                board_number.is_called = true;
                return true;
            }
        }
        println!("marking board {} for call {}", self.boardName, call);
        return false;
    }

    fn is_bingo(&self) -> bool {
        let board_numbers = self.boardNumbers.as_slice();
        // check if the board has full rows
        for y in 0..5 {
            if !(self.isCalled(y, 0)
                && self.isCalled(y, 1)
                && self.isCalled(y, 2)
                && self.isCalled(y, 3)
                && self.isCalled(y, 4)) {
                return false;
            }
        }

        // check if the board has full columns
        for x in 0..5 {
            if !(self.isCalled(0, x)
                && self.isCalled(1, x)
                && self.isCalled(2, x)
                && self.isCalled(3, x)
                && self.isCalled(4, x)) {
                return false;
            }
        }
        println!("BINGO ");
        true
    }
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

    let mut boards: Vec<Board> = Vec::new();

    let mut board_count = 0;
    // read the rest of the lines and generate the boards
    while let Some(_) = lines.next() {
        let mut line_count :i32 = 0;
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
                line_count += 1;
            }
        }
//        println!("data lines when flattened is {}", data_lines);
        if !board_numbers.is_empty() {
            let new_board = Board { boardNumbers: board_numbers, boardName: board_count.to_string() };
            &mut boards.push(new_board);
            board_count += 1;
        }
    }

    // feed the numbers into the boards and get them to check for a row/column
    for call in calls {
        println!("calling number {}", call);
        for board in boards.as_mut_slice() {
            let matches_call = board.accept_call(call);
            if matches_call {
                if board.is_bingo() {
                    println!("We have a winner");
                }
            }
        }
    }


    0
}

pub fn part_b(input: &str) -> i64 {
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
        assert_eq!(super::part_a(include_str!("realdata.txt")), -1);
    }

    #[test]
    fn part_b_test() {
        assert_eq!(super::part_b(include_str!("testdata.txt")), -1);
    }

    #[test]
    fn part_b_real() {
        assert_eq!(super::part_b(include_str!("realdata.txt")), -1);
    }
}
