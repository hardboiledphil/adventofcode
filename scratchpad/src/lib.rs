pub fn test_me() {
    let board1 = Board::new(false, [1, 3, 5, 7, 9]);
    let board2 = Board::new(false, [2, 4, 6, 8, 10]);

    let boards = Boards { list_of_boards: [board1, board2] };

    for mut board in boards.list_of_boards {
        println!("{:?}", board);
        board.call_me(42);
    }

//    println!("boards: {:?}", &boards);

    // assert!(boards.get(0).unwrap().all_called, false);

    // ref_mut_boards[0].all_called = true;

}

#[derive(Debug)]
pub struct Board {
    all_called: bool,
    numbers: [i32; 5]
}

impl Board {
    fn new(all_called: bool, numbers: [i32; 5]) -> Board {
        return Board {
            all_called,
            numbers,
        };
    }
    fn call_me(&mut self, number_to_call: i32) {
        if self.numbers.contains(&number_to_call) {
            self.all_called = true;
        }
    }
}

#[derive(Debug)]
pub struct Boards {
    list_of_boards: [Board; 2]
}

#[cfg(test)]
mod scratchpad_tests {
    #[test]
    fn scratchpad_test() {
        super::test_me();
    }


}