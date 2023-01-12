use teloxide::utils::command::ParseError;

pub fn parse_3_nl_args(
    input: String,
) -> Result<(String, String, String), ParseError> {
    let args = input.split('\n').collect::<Vec<_>>();
    match args.len() {
        3 => {
            if args[0].is_empty() || args[1].is_empty() || args[2].is_empty() {
                Err(ParseError::Custom("Arguments can't be empty".into()))
            } else {
                Ok((
                    args[0].to_string(),
                    args[1].to_string(),
                    args[2].to_string(),
                ))
            }
        }
        _ => Err(ParseError::Custom("Invalid number of arguments".into())),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_3_nl_args() {
        assert!(parse_3_nl_args("a".into()).is_err());
        assert!(parse_3_nl_args("a\nb".into()).is_err());
        assert!(parse_3_nl_args("\n\n".into()).is_err());
        assert_eq!(
            parse_3_nl_args("a\nb\nc".into()).unwrap(),
            ("a".into(), "b".into(), "c".into())
        );
    }
}
