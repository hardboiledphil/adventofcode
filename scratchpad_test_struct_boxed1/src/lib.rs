pub fn test_me() {
    let mut boxed_playground = Box::new(Playground { number: 42, my_list: vec![142] });
    increase_number_in_playground(&mut boxed_playground);
    println!("playground {:?}", boxed_playground);
    assert_eq!(boxed_playground.number, 43);
    assert_eq!(boxed_playground.my_list.get(0), Some(&1));
}

#[derive(Debug)]
struct Playground {
    number: i32,
    my_list: Vec<i32>
}

fn increase_number_in_playground(boxed_playground: &mut Box<Playground>) {
    boxed_playground.number += 1;
    boxed_playground.my_list.push(143);
}

#[cfg(test)]
mod scratchpad_tests {
    #[test]
    fn scratchpad_test_struct_boxed1() {
        super::test_me();
    }
}