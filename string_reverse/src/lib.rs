use std::ops::Index;

pub fn reverse(input: &str) -> String {

    // let mut new_string = String::new();
    // let str_length = input.len();
    // for i in 0..str_length {
    //     let x = str_length - i;
    //
    //     new_string.push_str(input.index(x-1..x));
    // }
    input.chars().rev().collect()
}

#[cfg(test)]
use time::PrimitiveDateTime as DateTime;
/// Create a datetime from the given numeric point in time.
///
/// Panics if any field is invalid.
fn dt(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> DateTime {
    use time::{Date, Time};
    DateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    )
}
#[test]
fn test_date() {
    let start_date = dt(2011, 4, 25, 0, 0, 0);
    assert_eq!(gigasecond::after(start_date), dt(2043, 1, 1, 1, 46, 40));
}
#[test]
#[ignore]
fn test_another_date() {
    let start_date = dt(1977, 6, 13, 0, 0, 0);
    assert_eq!(gigasecond::after(start_date), dt(2009, 2, 19, 1, 46, 40));
}
#[test]
#[ignore]
fn test_third_date() {
    let start_date = dt(1959, 7, 19, 0, 0, 0);
    assert_eq!(gigasecond::after(start_date), dt(1991, 3, 27, 1, 46, 40));
}
#[test]
#[ignore]
fn test_datetime() {
    let start_date = dt(2015, 1, 24, 22, 0, 0);
    assert_eq!(gigasecond::after(start_date), dt(2046, 10, 2, 23, 46, 40));
}
#[test]
#[ignore]
fn test_another_datetime() {
    let start_date = dt(2015, 1, 24, 23, 59, 59);
    assert_eq!(gigasecond::after(start_date), dt(2046, 10, 3, 1, 46, 39));
}