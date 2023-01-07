use std::path::Path;

use tool_versions::ast::{Identifier, Line, SyntaxError, Tool, Unparsed, Version, Whitespace, AST};
use tool_versions::{parser, transformer};

#[test]
fn it_sets_more_versions() {
    let ast = parser::parse_file(Path::new("tests/__fixtures__/_tool-versions")).unwrap();

    let result = transformer::set_versions(
        &ast,
        Identifier::new("nodejs"),
        vec![
            Identifier::new("7"),
            Identifier::new("9"),
            Identifier::new("10"),
        ],
    );

    assert_eq!(
        result,
        AST::new(vec![
            Line::Definition {
                tool: Tool::new(
                    Identifier::new("nodejs"),
                    vec![
                        Version::new(Identifier::new("7"), Whitespace::new("  ")),
                        Version::new(Identifier::new("9"), Whitespace::new("    ")),
                        Version::new(Identifier::new("10"), Whitespace::new(" ")),
                    ]
                ),
                whitespace: Some(Whitespace::new("  ")),
                comment: Some(Unparsed::new(" foobar  "))
            },
            Line::Definition {
                tool: Tool::new(
                    Identifier::new("ruby"),
                    vec![
                        Version::new(Identifier::new("12"), Whitespace::new("    ")),
                        Version::new(Identifier::new("19"), Whitespace::new("       ")),
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
                    vec![Version::new(Identifier::new("4"), Whitespace::new(" ")),]
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

#[test]
fn it_sets_less_versions() {
    let ast = parser::parse_file(Path::new("tests/__fixtures__/_tool-versions")).unwrap();

    let result =
        transformer::set_versions(&ast, Identifier::new("ruby"), vec![Identifier::new("14")]);

    assert_eq!(
        result,
        AST::new(vec![
            Line::Definition {
                tool: Tool::new(
                    Identifier::new("nodejs"),
                    vec![
                        Version::new(Identifier::new("18.12"), Whitespace::new("  ")),
                        Version::new(Identifier::new("system"), Whitespace::new("    ")),
                    ]
                ),
                whitespace: Some(Whitespace::new("  ")),
                comment: Some(Unparsed::new(" foobar  "))
            },
            Line::Definition {
                tool: Tool::new(
                    Identifier::new("ruby"),
                    vec![Version::new(Identifier::new("14"), Whitespace::new("    ")),]
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
                    vec![Version::new(Identifier::new("4"), Whitespace::new(" ")),]
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

#[test]
fn it_sets_new_versions() {
    let ast = parser::parse_file(Path::new("tests/__fixtures__/_tool-versions")).unwrap();

    let result = transformer::set_versions(
        &ast,
        Identifier::new("golang"),
        vec![Identifier::new("1337")],
    );

    assert_eq!(
        result,
        AST::new(vec![
            Line::Definition {
                tool: Tool::new(
                    Identifier::new("golang"),
                    vec![Version::new(Identifier::new("1337"), Whitespace::new(" ")),]
                ),
                whitespace: None,
                comment: None
            },
            Line::Definition {
                tool: Tool::new(
                    Identifier::new("nodejs"),
                    vec![
                        Version::new(Identifier::new("18.12"), Whitespace::new("  ")),
                        Version::new(Identifier::new("system"), Whitespace::new("    ")),
                    ]
                ),
                whitespace: Some(Whitespace::new("  ")),
                comment: Some(Unparsed::new(" foobar  "))
            },
            Line::Definition {
                tool: Tool::new(
                    Identifier::new("ruby"),
                    vec![
                        Version::new(Identifier::new("12"), Whitespace::new("    ")),
                        Version::new(Identifier::new("19"), Whitespace::new("       ")),
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
                    vec![Version::new(Identifier::new("4"), Whitespace::new(" ")),]
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

#[test]
fn it_sets_new_versions_on_empty() {
    let ast = AST::new(vec![]);

    let result = transformer::set_versions(
        &ast,
        Identifier::new("golang"),
        vec![Identifier::new("1337")],
    );

    assert_eq!(
        result,
        AST::new(vec![Line::Definition {
            tool: Tool::new(
                Identifier::new("golang"),
                vec![Version::new(Identifier::new("1337"), Whitespace::new(" ")),]
            ),
            whitespace: None,
            comment: None
        },]),
    )
}

#[test]
fn it_sets_new_empty_versions() {
    let ast = parser::parse_file(Path::new("tests/__fixtures__/_tool-versions")).unwrap();

    let result = transformer::set_versions(&ast, Identifier::new("golang"), vec![]);

    assert_eq!(
        result,
        AST::new(vec![
            Line::Definition {
                tool: Tool::new(
                    Identifier::new("nodejs"),
                    vec![
                        Version::new(Identifier::new("18.12"), Whitespace::new("  ")),
                        Version::new(Identifier::new("system"), Whitespace::new("    ")),
                    ]
                ),
                whitespace: Some(Whitespace::new("  ")),
                comment: Some(Unparsed::new(" foobar  "))
            },
            Line::Definition {
                tool: Tool::new(
                    Identifier::new("ruby"),
                    vec![
                        Version::new(Identifier::new("12"), Whitespace::new("    ")),
                        Version::new(Identifier::new("19"), Whitespace::new("       ")),
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
                    vec![Version::new(Identifier::new("4"), Whitespace::new(" ")),]
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

#[test]
fn it_removes_versions() {
    let ast = parser::parse_file(Path::new("tests/__fixtures__/_tool-versions")).unwrap();

    let result = transformer::set_versions(&ast, Identifier::new("ruby"), vec![]);

    assert_eq!(
        result,
        AST::new(vec![
            Line::Definition {
                tool: Tool::new(
                    Identifier::new("nodejs"),
                    vec![
                        Version::new(Identifier::new("18.12"), Whitespace::new("  ")),
                        Version::new(Identifier::new("system"), Whitespace::new("    ")),
                    ]
                ),
                whitespace: Some(Whitespace::new("  ")),
                comment: Some(Unparsed::new(" foobar  "))
            },
            Line::Empty {
                whitespace: Some(Whitespace::new("   ")),
                comment: Some(Unparsed::new("# foo ## bar "))
            },
            Line::Definition {
                tool: Tool::new(
                    Identifier::new("rust"),
                    vec![Version::new(Identifier::new("4"), Whitespace::new(" ")),]
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
