use std::path::Path;
use tool_versions::ast::{
    Identifier, Line, Node, SyntaxError, Unparsed, Version, Versions, Whitespace, AST,
};
use tool_versions::{parser, transformer};

#[test]
fn it_sets_more_versions() {
    let ast = parser::parse_file(Path::new("tests/__fixtures__/_tool-versions")).unwrap();

    let result = transformer::set_versions(
        &ast,
        Identifier::new("nodejs".to_string()),
        vec![
            Version::new("7".to_string()),
            Version::new("9".to_string()),
            Version::new("10".to_string()),
        ],
    );

    assert_eq!(
        result,
        AST {
            lines: vec![
                Line::ToolDefinition {
                    name: Identifier::new("nodejs".to_string()),
                    versions: Versions::new(vec![
                        (
                            Whitespace::new("  ".to_string()),
                            Version::new("7".to_string()),
                        ),
                        (
                            Whitespace::new("    ".to_string()),
                            Version::new("9".to_string()),
                        ),
                        (
                            Whitespace::new(" ".to_string()),
                            Version::new("10".to_string()),
                        ),
                    ]),
                    whitespace: Some(Whitespace::new("  ".to_string())),
                    comment: Some(Unparsed::new(" foobar  ".to_string()))
                },
                Line::ToolDefinition {
                    name: Identifier::new("ruby".to_string()),
                    versions: Versions::new(vec![
                        (
                            Whitespace::new("    ".to_string()),
                            Version::new("12".to_string())
                        ),
                        (
                            Whitespace::new("       ".to_string()),
                            Version::new("19".to_string())
                        ),
                    ]),
                    whitespace: None,
                    comment: None
                },
                Line::Empty {
                    whitespace: Some(Whitespace::new("   ".to_string())),
                    comment: Some(Unparsed::new("# foo ## bar ".to_string()))
                },
                Line::ToolDefinition {
                    name: Identifier::new("rust".to_string()),
                    versions: Versions::new(vec![(
                        Whitespace::new(" ".to_string()),
                        Version::new("4".to_string()),
                    ),]),
                    whitespace: Some(Whitespace::new("      ".to_string())),
                    comment: None
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedToken {
                        token: '+',
                        expected: "Identifier,Whitespace,Comment",
                    },
                    unparsed: Unparsed::new("+invalid 12 ".to_string()),
                },
                Line::Empty {
                    whitespace: Some(Whitespace::new("         ".to_string())),
                    comment: None
                },
                Line::Invalid {
                    error: SyntaxError::DuplicateIdentifier(Identifier::new("nodejs".to_string())),
                    unparsed: Unparsed::new("nodejs      12   ".to_string()),
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedToken {
                        token: 'i',
                        expected: "EOL,Comment",
                    },
                    unparsed: Unparsed::new(" ignored ".to_string()),
                },
                Line::Empty {
                    whitespace: None,
                    comment: Some(Unparsed::new(" asda".to_string()))
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedToken {
                        token: '#',
                        expected: "Whitespace",
                    },
                    unparsed: Unparsed::new("rust# comment".to_string()),
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedToken {
                        token: '+',
                        expected: "Whitespace",
                    },
                    unparsed: Unparsed::new("inva+lid 20".to_string()),
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedEOL {
                        expected: "Whitespace",
                    },
                    unparsed: Unparsed::new("rust".to_string()),
                },
                Line::ToolDefinition {
                    name: Identifier::new("lua".to_string()),
                    versions: Versions::new(vec![
                        (
                            Whitespace::new(" ".to_string()),
                            Version::new("19".to_string())
                        ),
                        (
                            Whitespace::new("      ".to_string()),
                            Version::new("20".to_string())
                        ),
                    ]),
                    whitespace: None,
                    comment: Some(Unparsed::new("ay".to_string()))
                },
                Line::Invalid {
                    error: SyntaxError::DuplicateIdentifier(Identifier::new("lua".to_string())),
                    unparsed: Unparsed::new("lua   ".to_string()),
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedEOL {
                        expected: "Version"
                    },
                    unparsed: Unparsed::new("golang ".to_string()),
                },
            ]
        },
    );
}

#[test]
fn it_sets_less_versions() {
    let ast = parser::parse_file(Path::new("tests/__fixtures__/_tool-versions")).unwrap();

    let result = transformer::set_versions(
        &ast,
        Identifier::new("ruby".to_string()),
        vec![Version::new("14".to_string())],
    );

    assert_eq!(
        result,
        AST {
            lines: vec![
                Line::ToolDefinition {
                    name: Identifier::new("nodejs".to_string()),
                    versions: Versions::new(vec![
                        (
                            Whitespace::new("  ".to_string()),
                            Version::new("18.12".to_string()),
                        ),
                        (
                            Whitespace::new("    ".to_string()),
                            Version::new("system".to_string()),
                        ),
                    ]),
                    whitespace: Some(Whitespace::new("  ".to_string())),
                    comment: Some(Unparsed::new(" foobar  ".to_string()))
                },
                Line::ToolDefinition {
                    name: Identifier::new("ruby".to_string()),
                    versions: Versions::new(vec![(
                        Whitespace::new("    ".to_string()),
                        Version::new("14".to_string())
                    ),]),
                    whitespace: None,
                    comment: None
                },
                Line::Empty {
                    whitespace: Some(Whitespace::new("   ".to_string())),
                    comment: Some(Unparsed::new("# foo ## bar ".to_string()))
                },
                Line::ToolDefinition {
                    name: Identifier::new("rust".to_string()),
                    versions: Versions::new(vec![(
                        Whitespace::new(" ".to_string()),
                        Version::new("4".to_string()),
                    ),]),
                    whitespace: Some(Whitespace::new("      ".to_string())),
                    comment: None
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedToken {
                        token: '+',
                        expected: "Identifier,Whitespace,Comment",
                    },
                    unparsed: Unparsed::new("+invalid 12 ".to_string()),
                },
                Line::Empty {
                    whitespace: Some(Whitespace::new("         ".to_string())),
                    comment: None
                },
                Line::Invalid {
                    error: SyntaxError::DuplicateIdentifier(Identifier::new("nodejs".to_string())),
                    unparsed: Unparsed::new("nodejs      12   ".to_string()),
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedToken {
                        token: 'i',
                        expected: "EOL,Comment",
                    },
                    unparsed: Unparsed::new(" ignored ".to_string()),
                },
                Line::Empty {
                    whitespace: None,
                    comment: Some(Unparsed::new(" asda".to_string()))
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedToken {
                        token: '#',
                        expected: "Whitespace",
                    },
                    unparsed: Unparsed::new("rust# comment".to_string()),
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedToken {
                        token: '+',
                        expected: "Whitespace",
                    },
                    unparsed: Unparsed::new("inva+lid 20".to_string()),
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedEOL {
                        expected: "Whitespace",
                    },
                    unparsed: Unparsed::new("rust".to_string()),
                },
                Line::ToolDefinition {
                    name: Identifier::new("lua".to_string()),
                    versions: Versions::new(vec![
                        (
                            Whitespace::new(" ".to_string()),
                            Version::new("19".to_string())
                        ),
                        (
                            Whitespace::new("      ".to_string()),
                            Version::new("20".to_string())
                        ),
                    ]),
                    whitespace: None,
                    comment: Some(Unparsed::new("ay".to_string()))
                },
                Line::Invalid {
                    error: SyntaxError::DuplicateIdentifier(Identifier::new("lua".to_string())),
                    unparsed: Unparsed::new("lua   ".to_string()),
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedEOL {
                        expected: "Version"
                    },
                    unparsed: Unparsed::new("golang ".to_string()),
                },
            ]
        },
    );
}

#[test]
fn it_sets_new_versions() {
    let ast = parser::parse_file(Path::new("tests/__fixtures__/_tool-versions")).unwrap();

    let result = transformer::set_versions(
        &ast,
        Identifier::new("golang".to_string()),
        vec![Version::new("1337".to_string())],
    );

    assert_eq!(
        result,
        AST {
            lines: vec![
                Line::ToolDefinition {
                    name: Identifier::new("nodejs".to_string()),
                    versions: Versions::new(vec![
                        (
                            Whitespace::new("  ".to_string()),
                            Version::new("18.12".to_string()),
                        ),
                        (
                            Whitespace::new("    ".to_string()),
                            Version::new("system".to_string()),
                        ),
                    ]),
                    whitespace: Some(Whitespace::new("  ".to_string())),
                    comment: Some(Unparsed::new(" foobar  ".to_string()))
                },
                Line::ToolDefinition {
                    name: Identifier::new("ruby".to_string()),
                    versions: Versions::new(vec![
                        (
                            Whitespace::new("    ".to_string()),
                            Version::new("12".to_string())
                        ),
                        (
                            Whitespace::new("       ".to_string()),
                            Version::new("19".to_string())
                        ),
                    ]),
                    whitespace: None,
                    comment: None
                },
                Line::Empty {
                    whitespace: Some(Whitespace::new("   ".to_string())),
                    comment: Some(Unparsed::new("# foo ## bar ".to_string()))
                },
                Line::ToolDefinition {
                    name: Identifier::new("rust".to_string()),
                    versions: Versions::new(vec![(
                        Whitespace::new(" ".to_string()),
                        Version::new("4".to_string()),
                    ),]),
                    whitespace: Some(Whitespace::new("      ".to_string())),
                    comment: None
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedToken {
                        token: '+',
                        expected: "Identifier,Whitespace,Comment",
                    },
                    unparsed: Unparsed::new("+invalid 12 ".to_string()),
                },
                Line::Empty {
                    whitespace: Some(Whitespace::new("         ".to_string())),
                    comment: None
                },
                Line::Invalid {
                    error: SyntaxError::DuplicateIdentifier(Identifier::new("nodejs".to_string())),
                    unparsed: Unparsed::new("nodejs      12   ".to_string()),
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedToken {
                        token: 'i',
                        expected: "EOL,Comment",
                    },
                    unparsed: Unparsed::new(" ignored ".to_string()),
                },
                Line::Empty {
                    whitespace: None,
                    comment: Some(Unparsed::new(" asda".to_string()))
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedToken {
                        token: '#',
                        expected: "Whitespace",
                    },
                    unparsed: Unparsed::new("rust# comment".to_string()),
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedToken {
                        token: '+',
                        expected: "Whitespace",
                    },
                    unparsed: Unparsed::new("inva+lid 20".to_string()),
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedEOL {
                        expected: "Whitespace",
                    },
                    unparsed: Unparsed::new("rust".to_string()),
                },
                Line::ToolDefinition {
                    name: Identifier::new("lua".to_string()),
                    versions: Versions::new(vec![
                        (
                            Whitespace::new(" ".to_string()),
                            Version::new("19".to_string())
                        ),
                        (
                            Whitespace::new("      ".to_string()),
                            Version::new("20".to_string())
                        ),
                    ]),
                    whitespace: None,
                    comment: Some(Unparsed::new("ay".to_string()))
                },
                Line::Invalid {
                    error: SyntaxError::DuplicateIdentifier(Identifier::new("lua".to_string())),
                    unparsed: Unparsed::new("lua   ".to_string()),
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedEOL {
                        expected: "Version"
                    },
                    unparsed: Unparsed::new("golang ".to_string()),
                },
                Line::ToolDefinition {
                    name: Identifier::new("golang".to_string()),
                    versions: Versions::new(vec![(
                        Whitespace::new(" ".to_string()),
                        Version::new("1337".to_string())
                    ),]),
                    whitespace: None,
                    comment: None
                },
            ]
        },
    );
}

#[test]
fn it_sets_new_versions_on_empty() {
    let ast = AST { lines: vec![] };

    let result = transformer::set_versions(
        &ast,
        Identifier::new("golang".to_string()),
        vec![Version::new("1337".to_string())],
    );

    assert_eq!(
        result,
        AST {
            lines: vec![Line::ToolDefinition {
                name: Identifier::new("golang".to_string()),
                versions: Versions::new(vec![(
                    Whitespace::new(" ".to_string()),
                    Version::new("1337".to_string()),
                ),]),
                whitespace: None,
                comment: None
            },]
        },
    )
}

#[test]
fn it_sets_new_empty_versions() {
    let ast = parser::parse_file(Path::new("tests/__fixtures__/_tool-versions")).unwrap();

    let result = transformer::set_versions(&ast, Identifier::new("golang".to_string()), vec![]);

    assert_eq!(result, ast);
}

#[test]
fn it_removes_versions() {
    let ast = parser::parse_file(Path::new("tests/__fixtures__/_tool-versions")).unwrap();

    let result = transformer::set_versions(&ast, Identifier::new("ruby".to_string()), vec![]);

    assert_eq!(
        result,
        AST {
            lines: vec![
                Line::ToolDefinition {
                    name: Identifier::new("nodejs".to_string()),
                    versions: Versions::new(vec![
                        (
                            Whitespace::new("  ".to_string()),
                            Version::new("18.12".to_string()),
                        ),
                        (
                            Whitespace::new("    ".to_string()),
                            Version::new("system".to_string()),
                        ),
                    ]),
                    whitespace: Some(Whitespace::new("  ".to_string())),
                    comment: Some(Unparsed::new(" foobar  ".to_string()))
                },
                Line::Empty {
                    whitespace: Some(Whitespace::new("   ".to_string())),
                    comment: Some(Unparsed::new("# foo ## bar ".to_string()))
                },
                Line::ToolDefinition {
                    name: Identifier::new("rust".to_string()),
                    versions: Versions::new(vec![(
                        Whitespace::new(" ".to_string()),
                        Version::new("4".to_string()),
                    ),]),
                    whitespace: Some(Whitespace::new("      ".to_string())),
                    comment: None
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedToken {
                        token: '+',
                        expected: "Identifier,Whitespace,Comment",
                    },
                    unparsed: Unparsed::new("+invalid 12 ".to_string()),
                },
                Line::Empty {
                    whitespace: Some(Whitespace::new("         ".to_string())),
                    comment: None
                },
                Line::Invalid {
                    error: SyntaxError::DuplicateIdentifier(Identifier::new("nodejs".to_string())),
                    unparsed: Unparsed::new("nodejs      12   ".to_string()),
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedToken {
                        token: 'i',
                        expected: "EOL,Comment",
                    },
                    unparsed: Unparsed::new(" ignored ".to_string()),
                },
                Line::Empty {
                    whitespace: None,
                    comment: Some(Unparsed::new(" asda".to_string()))
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedToken {
                        token: '#',
                        expected: "Whitespace",
                    },
                    unparsed: Unparsed::new("rust# comment".to_string()),
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedToken {
                        token: '+',
                        expected: "Whitespace",
                    },
                    unparsed: Unparsed::new("inva+lid 20".to_string()),
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedEOL {
                        expected: "Whitespace",
                    },
                    unparsed: Unparsed::new("rust".to_string()),
                },
                Line::ToolDefinition {
                    name: Identifier::new("lua".to_string()),
                    versions: Versions::new(vec![
                        (
                            Whitespace::new(" ".to_string()),
                            Version::new("19".to_string())
                        ),
                        (
                            Whitespace::new("      ".to_string()),
                            Version::new("20".to_string())
                        ),
                    ]),
                    whitespace: None,
                    comment: Some(Unparsed::new("ay".to_string()))
                },
                Line::Invalid {
                    error: SyntaxError::DuplicateIdentifier(Identifier::new("lua".to_string())),
                    unparsed: Unparsed::new("lua   ".to_string()),
                },
                Line::Invalid {
                    error: SyntaxError::UnexpectedEOL {
                        expected: "Version"
                    },
                    unparsed: Unparsed::new("golang ".to_string()),
                },
            ]
        },
    );
}
