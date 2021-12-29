pub fn test_me() {
    let mut boxed_playground = Box::new(Playground { number: 42 });
    increase_number_in_playground(&mut boxed_playground);
    println!("playground {:?}", boxed_playground);
}

#[derive(Debug)]
struct Playground {
    number: i32
}

fn increase_number_in_playground(boxed_playground: &mut Box<Playground>) {
    boxed_playground.number += 1;
}

#[cfg(test)]
mod scratchpad_tests {
    #[test]
    fn part_a_test() {
        super::test_me();
    }
}