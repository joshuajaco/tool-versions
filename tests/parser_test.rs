use std::{fs, path::Path};
use tool_versions::ast::{
    Identifier, Line, StringValue, SyntaxError, Tool, Unparsed, Version, Whitespace, AST,
};
use tool_versions::parser;

#[test]
fn it_works() {
    let path = Path::new("tests/__fixtures__/_tool-versions");

    let parse_file_result = parser::parse_file(path).unwrap();

    let input = fs::read_to_string(path).unwrap();
    let parse_result = parser::parse(&input);

    assert_eq!(parse_file_result, parse_result);

    assert_eq!(
        parse_file_result,
        AST::new(vec![
            Line::Definition {
                tool: Tool::new(
                    Identifier::new("nodejs"),
                    vec![
                        Version::new(StringValue::new("18.12"), Whitespace::new("  ")),
                        Version::new(StringValue::new("system"), Whitespace::new("    ")),
                    ]
                ),
                whitespace: Some(Whitespace::new("  ")),
                comment: Some(Unparsed::new(" foobar  "))
            },
            Line::Definition {
                tool: Tool::new(
                    Identifier::new("ruby"),
                    vec![
                        Version::new(StringValue::new("12"), Whitespace::new("    ")),
                        Version::new(StringValue::new("19"), Whitespace::new("       ")),
                    ]
                ),
                whitespace: None,
                comment: None
            },
            Line::Empty {
                whitespace: Some(Whitespace::new("   ")),
                comment: Some(Unparsed::new("# foo ## bar "))
            },
            Line::Definition {
                tool: Tool::new(
                    Identifier::new("rust"),
                    vec![Version::new(StringValue::new("4"), Whitespace::new(" ")),]
                ),
                whitespace: Some(Whitespace::new("      ")),
                comment: None
            },
            Line::Empty {
                whitespace: Some(Whitespace::new("         ")),
                comment: None
            },
            Line::Invalid {
                error: SyntaxError::DuplicateIdentifier("nodejs".to_string()),
                unparsed: Unparsed::new("nodejs      12   "),
            },
            Line::Invalid {
                error: SyntaxError::UnexpectedToken {
                    token: 'i',
                    expected: "EOL,Comment",
                },
                unparsed: Unparsed::new(" ignored "),
            },
            Line::Empty {
                whitespace: None,
                comment: Some(Unparsed::new(" asda"))
            },
            Line::Invalid {
                error: SyntaxError::UnexpectedToken {
                    token: '#',
                    expected: "Version",
                },
                unparsed: Unparsed::new("rust# comment"),
            },
            Line::Invalid {
                error: SyntaxError::UnexpectedToken {
                    token: '+',
                    expected: "Whitespace",
                },
                unparsed: Unparsed::new("inva+lid 20"),
            },
            Line::Invalid {
                error: SyntaxError::UnexpectedEOL {
                    expected: "Version",
                },
                unparsed: Unparsed::new("rust"),
            },
            Line::Definition {
                tool: Tool::new(
                    Identifier::new("lua"),
                    vec![
                        Version::new(StringValue::new("19"), Whitespace::new(" ")),
                        Version::new(StringValue::new("20"), Whitespace::new("      ")),
                    ]
                ),
                whitespace: None,
                comment: Some(Unparsed::new("ay"))
            },
            Line::Invalid {
                error: SyntaxError::DuplicateIdentifier("lua".to_string()),
                unparsed: Unparsed::new("lua   "),
            },
            Line::Invalid {
                error: SyntaxError::UnexpectedEOL {
                    expected: "Version"
                },
                unparsed: Unparsed::new("golang "),
            },
        ]),
    )
}
