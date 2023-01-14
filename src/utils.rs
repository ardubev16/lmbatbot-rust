use teloxide::utils::command::ParseError;

pub fn parse_tagadd_args(
    input: String,
) -> Result<(String, String, Vec<String>), ParseError> {
    let args = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();
    match args.len() {
        3 => {
            let (group, emoji, names) =
                (args[0].clone(), args[1].clone(), args[2].clone());
            Ok((
                group,
                emoji,
                names
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
                    .collect(),
            ))
        }
        n_args => {
            if n_args > 3 {
                Err(ParseError::TooManyArguments {
                    expected: 3,
                    found: n_args,
                    message: "Too many arguments".into(),
                })
            } else {
                Err(ParseError::TooFewArguments {
                    expected: 3,
                    found: n_args,
                    message: "Too few arguments".into(),
                })
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_3_nl_args() {
        assert!(parse_tagadd_args("a".into()).is_err());
        assert!(parse_tagadd_args("a\nb".into()).is_err());
        assert!(parse_tagadd_args("\n\n".into()).is_err());
        assert_eq!(
            parse_tagadd_args("a\nb\n\nc".into()).unwrap(),
            ("a".into(), "b".into(), vec!["c".into()])
        );
        assert_eq!(
            parse_tagadd_args("a\nb\nc b".into()).unwrap(),
            ("a".into(), "b".into(), vec!["c".into(), "b".into()])
        );
        assert_eq!(
            parse_tagadd_args("a\nb\nc ".into()).unwrap(),
            ("a".into(), "b".into(), vec!["c".into()])
        );
        assert_eq!(
            parse_tagadd_args("a\nb\nc  d".into()).unwrap(),
            ("a".into(), "b".into(), vec!["c".into(), "d".into()])
        );
    }
}
