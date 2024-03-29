#[cfg(test)]
mod previous_crashes {
    use japanese_number_parser::JapaneseNumberFormatter;

    #[test]
    fn crashes() {
        let formatter = JapaneseNumberFormatter::new();

        assert_eq!(formatter.format("Д"), None);
        assert_eq!(formatter.format(" 億0"), None);
        assert_eq!(
            formatter
                .format("0分0分.11111111111111111111111111111111111111111111111111111111111111111"),
            None
        );
        assert_eq!(formatter.format("四割"), None);
    }
}
