pub fn interpret(command: String) -> String {
    command.replace("(al)", "al").replace("()", "o")
}

#[cfg(test)]
mod tests {
    use crate::strings::goal_parser_interpretation::interpret;

    #[test]
    fn test_interpret() {
        assert_eq!(interpret("G()(al)".to_string()), "Goal".to_string());
        assert_eq!(
            interpret("G()()()()(al)".to_string()),
            "Gooooal".to_string()
        );
        assert_eq!(
            interpret("(al)G(al)()()G".to_string()),
            "alGalooG".to_string()
        );
    }
}
