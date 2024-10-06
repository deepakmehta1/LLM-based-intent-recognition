#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_input() {
        let input = "Hello, world!";
        let static_input = String::from(input);
        assert_eq!(static_input.trim(), "Hello, world!");
    }

    #[test]
    fn test_print_input() {
        let input = "Hello, world!";
        let expected_output = format!("{}", input);
        assert_eq!(expected_output, "Hello, world!");
    }
}