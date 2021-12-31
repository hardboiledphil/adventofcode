const NUMBER: &str = "number:";

pub fn test_me() {

    let playground1 = Playground { number: 42, name: "number:42".to_string() };
    let playground2 = Playground { number: 422, name: "number:422".to_string() };
    let playground3 = Playground { number: 4222, name: "number:4222".to_string() };

    let playgrounds: &mut Vec<_> = &mut Vec::new();
    playgrounds.push(playground1);
    playgrounds.push(playground2);

    increase_number_in_playground(playgrounds.get_mut(0));
    update_name_to_match_number(playgrounds.get_mut(0));
    assert_eq!(playgrounds.get(0).unwrap().number, 43);
    playgrounds.push(playground3);
    increase_number_in_playground(playgrounds.get_mut(1));
    update_name_to_match_number(playgrounds.get_mut(1));
    assert_eq!(playgrounds.get(1).unwrap().number, 423);
    increase_number_in_playground(playgrounds.get_mut(2));
    update_name_to_match_number(playgrounds.get_mut(2));
    assert_eq!(playgrounds.get(2).unwrap().number, 4223);
    println!("playground {:?}", playgrounds);
}

#[derive(Debug)]
struct Playground {
    number: i32,
    name: String,
}

fn increase_number_in_playground(playground: Option<&mut Playground>) {
    match playground {
        None => {}
        Some(_) => { playground.unwrap().number += 1; }
    }
}
fn update_name_to_match_number(playground: Option<&mut Playground>) {
    match playground {
        None => {}
        Some(_) => {
            let playground_ref = playground.unwrap();
            playground_ref.name = NUMBER.to_owned() + playground_ref.number.to_string().as_str();
        }
    }
}

#[cfg(test)]
mod scratchpad_vector_of_medium_structs_tests {
    #[test]
    fn scratchpad_vector_of_medium_structs_test() {
        super::test_me();
    }
}