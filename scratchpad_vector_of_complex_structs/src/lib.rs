const NUMBER: &str = "number:";

pub fn test_me() {

    let numbers1 = Numbers { myfoo: vec![1, 2, 3, 4]};
    let numbers2 = Numbers { myfoo: vec![2, 4, 6, 8, 10]};
    let numbers3 = Numbers { myfoo: vec![5, 6, 7, 8, 9, 10, 11, 12]};

    let position1 = Position { y: 1, x: 2 };
    let position2 = Position { y: 2, x: 3 };
    let position3 = Position { y: 3, x: 4 };

    let playground1 = Playground { value: 42, name: "number:42".to_string(), numbers: numbers1, pair: (1, 2), position: position1 };
    let playground2 = Playground { value: 422, name: "number:422".to_string(), numbers: numbers2, pair: (2, 3), position: position2 };
    let playground3 = Playground { value: 4222, name: "number:4222".to_string(), numbers: numbers3, pair: (3, 4), position: position3 };

    let playgrounds: &mut Vec<_> = &mut Vec::new();
    playgrounds.push(playground1);
    playgrounds.push(playground2);

    increase_number_in_playground(playgrounds.get_mut(0));
    update_name_to_match_number(playgrounds.get_mut(0));
    increase_numbers_in_vector(playgrounds.get_mut(0));
    increase_numbers_in_pair(playgrounds.get_mut(0));
    increase_numbers_in_position(playgrounds.get_mut(0));
    assert_eq!(playgrounds.get(0).unwrap().value, 43);
    playgrounds.push(playground3);
    increase_number_in_playground(playgrounds.get_mut(1));
    update_name_to_match_number(playgrounds.get_mut(1));
    increase_numbers_in_vector(playgrounds.get_mut(1));
    increase_numbers_in_pair(playgrounds.get_mut(1));
    increase_numbers_in_position(playgrounds.get_mut(1));
    assert_eq!(playgrounds.get(1).unwrap().value, 423);
    increase_number_in_playground(playgrounds.get_mut(2));
    update_name_to_match_number(playgrounds.get_mut(2));
    increase_numbers_in_vector(playgrounds.get_mut(2));
    increase_numbers_in_pair(playgrounds.get_mut(2));
    increase_numbers_in_position(playgrounds.get_mut(2));
    assert_eq!(playgrounds.get(2).unwrap().value, 4223);
    println!("playground {:?}", playgrounds);
}

#[derive(Debug)]
struct Playground {
    value: i32,
    name: String,
    numbers: Numbers,
    pair: (i32, i32),
    position: Position,
}

#[derive(Debug)]
struct Numbers {
    myfoo: Vec<i32>,
}

#[derive(Debug)]
struct Position {
    y: i32,
    x: i32,
}

fn increase_numbers_in_position(playground: Option<&mut Playground>) {
    match playground {
        None => {}
        Some(_) => {
            let mut my_playground = playground.unwrap();
            my_playground.position.y += 1;
            my_playground.position.x += 1;
        }
    }
}

fn increase_numbers_in_pair(playground: Option<&mut Playground>) {
    match playground {
        None => {}
        Some(_) => {
            let mut my_playground = playground.unwrap();
            my_playground.pair.0 += 1;
            my_playground.pair.1 += 1;
        }
    }
}

fn increase_numbers_in_vector(playground: Option<&mut Playground>) {
    match playground {
        None => {}
        Some(_) => {
            playground.unwrap().numbers.myfoo.iter_mut()
                .for_each(|x| *x += 1);
        }
    }
}

fn increase_number_in_playground(playground: Option<&mut Playground>) {
    match playground {
        None => {}
        Some(_) => { playground.unwrap().value += 1; }
    }
}
fn update_name_to_match_number(playground: Option<&mut Playground>) {
    match playground {
        None => {}
        Some(_) => {
            let playground_ref = playground.unwrap();
            playground_ref.name = NUMBER.to_owned() + playground_ref.value.to_string().as_str();
        }
    }
}

#[cfg(test)]
mod scratchpad_vector_of_complex_structs_tests {
    #[test]
    fn scratchpad_vector_of_complex_structs_test() {
        super::test_me();
    }
}