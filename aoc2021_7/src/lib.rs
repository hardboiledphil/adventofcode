/*
--- Day 7: The Treachery of Whales ---

A giant whale has decided your submarine is its next meal, and it's much faster than you are. There's nowhere to run!

Suddenly, a swarm of crabs (each in its own tiny submarine - it's too deep for them otherwise) zooms in to rescue you! They seem to be preparing to blast a hole in the ocean floor; sensors indicate a massive underground cave system just beyond where they're aiming!

The crab submarines all need to be aligned before they'll have enough power to blast a large enough hole for your submarine to get through. However, it doesn't look like they'll be aligned before the whale catches you! Maybe you can help?

There's one major catch - crab submarines can only move horizontally.

You quickly make a list of the horizontal position of each crab (your puzzle input). Crab submarines have limited fuel, so you need to find a way to make all of their horizontal positions match while requiring them to spend as little fuel as possible.

For example, consider the following horizontal positions:

16,1,2,0,4,2,7,1,2,14
This means there's a crab with horizontal position 16, a crab with horizontal position 1, and so on.

Each change of 1 step in horizontal position of a single crab costs 1 fuel. You could choose any horizontal position to align them all on, but the one that costs the least fuel is horizontal position 2:

Move from 16 to 2: 14 fuel
Move from 1 to 2: 1 fuel
Move from 2 to 2: 0 fuel
Move from 0 to 2: 2 fuel
Move from 4 to 2: 2 fuel
Move from 2 to 2: 0 fuel
Move from 7 to 2: 5 fuel
Move from 1 to 2: 1 fuel
Move from 2 to 2: 0 fuel
Move from 14 to 2: 12 fuel
This costs a total of 37 fuel. This is the cheapest possible outcome; more expensive outcomes include aligning at position 1 (41 fuel), position 3 (39 fuel), or position 10 (71 fuel).

Determine the horizontal position that the crabs can align to using the least fuel possible. How much fuel must they spend to align to that position?
 */

use std::collections::HashMap;

pub fn part_a(input: &str) -> i32 {

    let mut crab_positions: Vec<i32> = Vec::new();

    let lines = input.trim().split(',');

    // get a sorted array of all the crab positions
    for line in lines {
        let crab_position = line.parse().unwrap();
        crab_positions.push(crab_position);
    }

    // for each distinct crab position work out the fuel cost for all crabs to get to that position.
    // add to hashmap of position -> fuel cost
    let mut results: HashMap<i32, i32> = HashMap::new();
    for crab_position in crab_positions.as_slice().into_iter() {
        let result = calculate_fuel_a(&crab_position, &crab_positions);
        println!("crab_position, result {},{}", *crab_position, result.to_string());
        results.insert(result, *crab_position);
    }

    // what's the lowest value in the results hashmap
    let lowest_fuel = results.keys().min().unwrap();
    // get the position linked to this key
    *lowest_fuel
}

fn calculate_fuel_a(crab_position: &i32, crab_positions: &Vec<i32>) -> i32 {

    let mut fuel_amount = 0 as i32;
    for number in crab_positions {
        fuel_amount += (crab_position - number).abs();
    }
    fuel_amount
}

/*
--- Part Two ---

The crabs don't seem interested in your proposed solution. Perhaps you misunderstand crab
engineering?

As it turns out, crab submarine engines don't burn fuel at a constant rate. Instead, each change of
1 step in horizontal position costs 1 more unit of fuel than the last: the first step costs 1, the
second step costs 2, the third step costs 3, and so on.

As each crab moves, moving further becomes more expensive. This changes the best horizontal
position to align them all on; in the example above, this becomes 5:

Move from 16 to 5: 66 fuel
Move from 1 to 5: 10 fuel
Move from 2 to 5: 6 fuel
Move from 0 to 5: 15 fuel
Move from 4 to 5: 1 fuel
Move from 2 to 5: 6 fuel
Move from 7 to 5: 3 fuel
Move from 1 to 5: 10 fuel
Move from 2 to 5: 6 fuel
Move from 14 to 5: 45 fuel
This costs a total of 168 fuel. This is the new cheapest possible outcome; the old alignment position (2) now costs 206 fuel instead.

Determine the horizontal position that the crabs can align to using the least fuel possible so they can make you an escape route! How much fuel must they spend to align to that position?

 */

pub fn part_b(input: &str) -> i32 {

    let mut crab_positions: Vec<i32> = Vec::new();

    let lines = input.trim().split(',');

    // get a sorted array of all the crab positions
    for line in lines {
        let crab_position = line.parse().unwrap();
        crab_positions.push(crab_position);
    }

    // for each distinct crab position work out the fuel cost for all crabs to get to that position.
    // add to hashmap of position -> fuel cost
    let mut results: HashMap<i32, i32> = HashMap::new();
    for crab_position in crab_positions.as_slice().into_iter() {
        let result = calculate_fuel_b(&crab_position, &crab_positions);
        println!("crab_position, result {},{}", *crab_position, result.to_string());
        results.insert(result, *crab_position);
    }

    // what's the lowest value in the results hashmap
    let lowest_fuel = results.keys().min().unwrap();
    // get the position linked to this key
    *lowest_fuel
}

pub fn part_b_1(input: &str) -> i32 {

    let mut crab_positions: Vec<i32> = Vec::new();

    let lines = input.trim().split(',');

    // get a sorted array of all the crab positions
    for line in lines {
        let crab_position = line.parse().unwrap();
        crab_positions.push(crab_position);
    }

    // for each distinct crab position work out the fuel cost for all crabs to get to that position.
    // add to hashmap of position -> fuel cost
    let value_to_check = 491 as i32;
    let result = calculate_fuel_b(&value_to_check, &crab_positions);
    println!("crab_position, result {}", result.to_string());

    result
}

fn calculate_fuel_b(crab_position: &i32, crab_positions: &Vec<i32>) -> i32 {

    let mut fuel_amount = 0 as i32;
    for number in crab_positions {
        let distance = (crab_position - number).abs();
        fuel_amount += fuel_for_distance(distance);
    }
    fuel_amount
}

fn fuel_for_distance(distance: i32) -> i32 {
    let mut fuel_for_distance = 0;
    for step in 1..=distance {
        fuel_for_distance += step;
    }
    fuel_for_distance
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a_test() {
        assert_eq!(super::part_a(include_str!("testdata.txt")), 37);
    }

    #[test]
    fn part_a_real() {
        assert_eq!(super::part_a(include_str!("realdata.txt")), 356992);
    }

    #[test]
    fn part_b_test() {
        assert_eq!(super::part_b(include_str!("testdata.txt")), 168);
    }

    #[test]
    fn part_b_real() {
        assert_eq!(super::part_b(include_str!("realdata.txt")), -1);
    }

    #[test]
    fn part_b_1_real() {
        assert_eq!(super::part_b_1(include_str!("realdata.txt")), -1);
        // 487, 101,271,050
        // 488, 101,269,080
        // 489, 101,268,110
        // 490, 101,268,141
        // 491, 101,269,172
    }

    #[test]
    fn distance_0() {
        assert_eq!(super::fuel_for_distance(0), 0);
    }

    #[test]
    fn distance_1() {
        assert_eq!(super::fuel_for_distance(1), 1);
    }

    #[test]
    fn distance_2() {
        assert_eq!(super::fuel_for_distance(2), 3);
    }

    #[test]
    fn distance_3() {
        assert_eq!(super::fuel_for_distance(3), 6);
    }

}
