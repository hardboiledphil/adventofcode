
#[cfg(test)]
mod tests {


    #[test]
    fn test_black() {
        assert_eq!(super::color_to_value(super::ResistorColor::Black), 0);
    }


}
