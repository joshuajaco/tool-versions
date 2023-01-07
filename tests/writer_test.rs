use std::{env, fs, path::Path};
use tool_versions::ast::{Identifier, Line, SyntaxError, Tool, Unparsed, Version, Whitespace, AST};
use tool_versions::writer;

#[test]
fn it_works() {
    let ast = AST::new(vec![
        Line::Definition {
            tool: Tool::new(
                Identifier::new("nodejs"),
                vec![
                    Version::new(Identifier::new("18.12"), Whitespace::new("  ")),
                    Version::new(Identifier::new("system"), Whitespace::new("    ")),
                ],
            ),
            whitespace: Some(Whitespace::new("  ")),
            comment: Some(Unparsed::new(" foobar  ")),
        },
        Line::Definition {
            tool: Tool::new(
                Identifier::new("ruby"),
                vec![
                    Version::new(Identifier::new("12"), Whitespace::new("    ")),
                    Version::new(Identifier::new("19"), Whitespace::new("       ")),
                ],
            ),
            whitespace: None,
            comment: None,
        },
        Line::Empty {
            whitespace: Some(Whitespace::new("   ")),
            comment: Some(Unparsed::new("# foo ## bar ")),
        },
        Line::Definition {
            tool: Tool::new(
                Identifier::new("rust"),
                vec![Version::new(Identifier::new("4"), Whitespace::new(" "))],
            ),
            whitespace: Some(Whitespace::new("      ")),
            comment: None,
        },
        Line::Empty {
            whitespace: Some(Whitespace::new("         ")),
            comment: None,
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
            comment: Some(Unparsed::new(" asda")),
        },
        Line::Invalid {
            error: SyntaxError::UnexpectedToken {
                token: '#',
                expected: "Version",
            },
            unparsed: Unparsed::new("rust# comment"),
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
                    Version::new(Identifier::new("19"), Whitespace::new(" ")),
                    Version::new(Identifier::new("20"), Whitespace::new("      ")),
                ],
            ),
            whitespace: None,
            comment: Some(Unparsed::new("ay")),
        },
        Line::Invalid {
            error: SyntaxError::DuplicateIdentifier("lua".to_string()),
            unparsed: Unparsed::new("lua   "),
        },
        Line::Invalid {
            error: SyntaxError::UnexpectedEOL {
                expected: "Version",
            },
            unparsed: Unparsed::new("golang "),
        },
    ]);

    let expected = fs::read_to_string(Path::new("tests/__fixtures__/_tool-versions")).unwrap();

    let path = env::temp_dir().join("_tool-versions");

    writer::write_file(&ast, path.clone()).unwrap();

    let result = fs::read_to_string(path).unwrap();

    assert_eq!(result, expected)
}
