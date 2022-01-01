
pub fn test_me() {

    let playground1 = Playground { number: 42 };
    let playground2 = Playground { number: 422 };
    let playground3 = Playground { number: 4222 };

    let playgrounds: &mut Vec<_> = &mut Vec::new();
    playgrounds.push(playground1);
    playgrounds.push(playground2);

    increase_number_in_playground(playgrounds.get_mut(0));
    assert_eq!(playgrounds.get(0).unwrap().number, 43);
    playgrounds.push(playground3);
    increase_number_in_playground(playgrounds.get_mut(1));
    assert_eq!(playgrounds.get(1).unwrap().number, 423);
    increase_number_in_playground(playgrounds.get_mut(2));
    assert_eq!(playgrounds.get(2).unwrap().number, 4223);
    println!("playground {:?}", playgrounds);
}

#[derive(Debug)]
struct Playground {
    number: i32
}

fn increase_number_in_playground(playground: Option<&mut Playground>) {
    if let Some(playground_unwrapped) = playground {
        playground_unwrapped.number += 1; }
}

#[cfg(test)]
mod scratchpad_vector_of_simple_structs_tests {
    #[test]
    fn scratchpad_vector_of_simple_structs_test() {
        super::test_me();
    }
}