
pub fn test_me() {

    let playground1 = Playground { number: 42 };
    let playground2 = Playground { number: 422 };
    let playground3 = Playground { number: 4222 };

    let playgrounds: &mut Vec<_> = &mut Vec::new();
    playgrounds.push(playground1);
    playgrounds.push(playground2);

    increase_number_in_playground(playgrounds);
//    assert_eq!(playground1.number, 43);
    playgrounds.push(playground3);
    // increase_number_in_playground(playgrounds.get_mut(2).unwrap());
    // assert_eq!(playground3.number, 4223);
    // println!("playground {:?}", playgrounds);
}

#[derive(Debug)]
struct Playground {
    number: i32
}

fn increase_number_in_playground(playground: &mut Vec<Playground>) {
    let iter = playground.iter_mut();
    for iter_playground in iter {
        iter_playground.number +=1;
    }
}

#[cfg(test)]
mod scratchpad_vector_of_simple_structs_tests {
    #[test]
    fn scratchpad_vector_of_simple_structs_test() {
        super::test_me();
    }
}