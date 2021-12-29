pub fn test_me() {
    let mut playground = Playground { number: 42 };
    increase_number_in_playground(&mut playground);
    println!("playground {:?}", playground);
}

#[derive(Debug)]
struct Playground {
    number: i32
}

fn increase_number_in_playground(playground: &mut Playground) {
    playground.number += 1;
}

#[cfg(test)]
mod scratchpad_tests {
    #[test]
    fn part_a_test() {
        super::test_me();
    }
}