/*
--- Day 5: Hydrothermal Venture ---

You come across a field of hydrothermal vents on the ocean floor! These vents constantly produce
large, opaque clouds, so it would be best to avoid them if possible.

They tend to form in lines; the submarine helpfully produces a list of nearby lines of vents (your
puzzle input) for you to review. For example:

0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
Each line of vents is given as a line segment in the format x1,y1 -> x2,y2 where x1,y1 are the
coordinates of one end the line segment and x2,y2 are the coordinates of the other end. These line
segments include the points at both ends. In other words:

An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.
For now, only consider horizontal and vertical lines: lines where either x1 = x2 or y1 = y2.

So, the horizontal and vertical lines from the above list would produce the following diagram:

.......1..
..1....1..
..1....1..
.......1..
.112111211
..........
..........
..........
..........
222111....
In this diagram, the top left corner is 0,0 and the bottom right corner is 9,9. Each position is
shown as the number of lines which cover that point or . if no line covers that point. The top-left
pair of 1s, for example, comes from 2,2 -> 2,1; the very bottom row is formed by the overlapping
lines 0,9 -> 5,9 and 0,9 -> 2,9.

To avoid the most dangerous areas, you need to determine the number of points where at least two
lines overlap. In the above example, this is anywhere in the diagram with a 2 or larger - a total
of 5 points.

Consider only horizontal and vertical lines. At how many points do at least two lines overlap?

 */

use std::collections::HashMap;

pub fn part_a(input: &str) -> i64 {
    let mut data_map: HashMap<(i64, i64), i64>= HashMap::new();

    let lines = input.trim().split('\n');

    for line in lines {
        let mut parts = line.split(" -> ");
        let start: Vec<i64> = parts
            .next()
            .unwrap()
            .split(",")
            .map(|value| value.parse().unwrap())
            .collect();
        let end: Vec<i64> = parts
            .next()
            .unwrap()
            .split(",")
            .map(|value| value.parse().unwrap())
            .collect();

        let start_x = start[0];
        let start_y = start[1];
        let end_x = end[0];
        let end_y = end[1];

        // where the x's are the same then process the y's
        if start_x == end_x {
            // iterate through the y's
            println!("looking at x:{} from: {} to: {}", start_x, start_y.min(end_y), end_y.max(start_y));
            for y in start_y.min(end_y)..=end_y.max(start_y) {
                // println!("plotting {},{}", start_x, y);
                *data_map.entry((start_x, y)).or_default() += 1;
            }
        }

        // where the y's are the same then process the x's
        if start_y == end_y {
            // iterate through the x's
            println!("looking at y:{} from: {} to: {}", start_y, start_x.min(end_x), end_x.max(start_x));
            for x in start_x.min(end_x)..=end_x.max(start_x) {
                // println!("plotting {}, {}", start_y, x);
                *data_map.entry((x, start_y)).or_default() += 1;
            }
        }

    }

    data_map.values().filter(|value| **value > 1).count() as i64
}

pub fn part_b(input: &str) -> i64 {
    let mut data_map: HashMap<(i64, i64), i64>= HashMap::new();

    let lines = input.trim().split('\n');

    for line in lines {
        let mut parts = line.split(" -> ");
        let start: Vec<i64> = parts
            .next()
            .unwrap()
            .split(",")
            .map(|value| value.parse().unwrap())
            .collect();
        let end: Vec<i64> = parts
            .next()
            .unwrap()
            .split(",")
            .map(|value| value.parse().unwrap())
            .collect();

        let start_x = start[0];
        let start_y = start[1];
        let end_x = end[0];
        let end_y = end[1];

        // where the x's are the same then process the y's
        if start_x == end_x {
            // iterate through the y's
            // println!("looking at x:{} from: {} to: {}", start_x, start_y.min(end_y), end_y.max(start_y));
            for y in start_y.min(end_y)..=end_y.max(start_y) {
                // println!("plotting {},{}", start_x, y);
                *data_map.entry((start_x, y)).or_default() += 1;
            }
        }

        // where the y's are the same then process the x's
        if start_y == end_y {
            // iterate through the x's
            // println!("looking at y:{} from: {} to: {}", start_y, start_x.min(end_x), end_x.max(start_x));
            for x in start_x.min(end_x)..=end_x.max(start_x) {
                // println!("plotting {}, {}", start_y, x);
                *data_map.entry((x, start_y)).or_default() += 1;
            }
        }

        let horizontal = (start_x - end_x).abs();
        let vertical = (start_y - end_y).abs();

        // println!("  horizontal {} vertical {}", horizontal, vertical);

        // // if we can tell the values are on a diagonal then process them
        if horizontal == vertical {
            // println!("  Processing diagonal from {},{} to {},{}",
            //     start_x, start_y, end_x, end_y);
            let x_direction = if start_x - end_x > 0 { -1 } else { 1 };
            let y_direction = if start_y - end_y > 0 { -1 } else { 1 };
            // println!("  x_dir {} y_dir {}", x_direction, y_direction);

            let mut latest_x = start_x;
            let mut latest_y= start_y;
            // println!("  latest_x {} latest_y {}", latest_x, latest_y);
            *data_map.entry((latest_x, latest_y)).or_default() += 1;
            while latest_x != end_x {
                // println!("looking at: {},{}", latest_x, latest_y);
                latest_x += x_direction;
                latest_y += y_direction;
                *data_map.entry((latest_x, latest_y)).or_default() += 1;
            }
            assert!(latest_y == end_y);
        }
    }

    data_map.values().filter(|value| **value > 1).count() as i64
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a_test() {
        assert_eq!(super::part_a(include_str!("testdata.txt")), 5);
    }

    #[test]
    fn part_a_real() {
        assert_eq!(super::part_a(include_str!("realdata.txt")), 5690);
    }

    #[test]
    fn part_b_test() {
        assert_eq!(super::part_b(include_str!("testdata.txt")), 12);
    }

    #[test]
    fn part_b_real() {
        assert_eq!(super::part_b(include_str!("realdata.txt")), 17741);
    }
}
