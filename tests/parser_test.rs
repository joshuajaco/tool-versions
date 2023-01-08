use std::{fs, path::Path};
use tool_versions::ast::{
    Identifier, Line, SyntaxError, Unparsed, Version, VersionString, Versions, Whitespace, AST,
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
            Line::ToolDefinition {
                name: Identifier::new("nodejs"),
                versions: Versions::new(vec![
                    Version::new(VersionString::new("18.12"), Whitespace::new("  ")),
                    Version::new(VersionString::new("system"), Whitespace::new("    ")),
                ]),
                whitespace: Some(Whitespace::new("  ")),
                comment: Some(Unparsed::new(" foobar  "))
            },
            Line::ToolDefinition {
                name: Identifier::new("ruby"),
                versions: Versions::new(vec![
                    Version::new(VersionString::new("12"), Whitespace::new("    ")),
                    Version::new(VersionString::new("19"), Whitespace::new("       ")),
                ]),
                whitespace: None,
                comment: None
            },
            Line::Empty {
                whitespace: Some(Whitespace::new("   ")),
                comment: Some(Unparsed::new("# foo ## bar "))
            },
            Line::ToolDefinition {
                name: Identifier::new("rust"),
                versions: Versions::new(vec![Version::new(
                    VersionString::new("4"),
                    Whitespace::new(" ")
                ),]),
                whitespace: Some(Whitespace::new("      ")),
                comment: None
            },
            Line::Invalid {
                error: SyntaxError::UnexpectedToken {
                    token: '+',
                    expected: "Identifier,Whitespace,Comment",
                },
                unparsed: Unparsed::new("+invalid 12 "),
            },
            Line::Empty {
                whitespace: Some(Whitespace::new("         ")),
                comment: None
            },
            Line::Invalid {
                error: SyntaxError::DuplicateIdentifier(Identifier::new("nodejs")),
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
            Line::ToolDefinition {
                name: Identifier::new("lua"),
                versions: Versions::new(vec![
                    Version::new(VersionString::new("19"), Whitespace::new(" ")),
                    Version::new(VersionString::new("20"), Whitespace::new("      ")),
                ]),
                whitespace: None,
                comment: Some(Unparsed::new("ay"))
            },
            Line::Invalid {
                error: SyntaxError::DuplicateIdentifier(Identifier::new("lua")),
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
