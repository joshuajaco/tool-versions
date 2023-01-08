use std::path::Path;

use tool_versions::ast::{
    Identifier, Line, SyntaxError, Unparsed, Version, VersionString, Versions, Whitespace, AST,
};
use tool_versions::{parser, transformer};

#[test]
fn it_sets_more_versions() {
    let ast = parser::parse_file(Path::new("tests/__fixtures__/_tool-versions")).unwrap();

    let result = transformer::set_versions(
        &ast,
        Identifier::new("nodejs"),
        vec![
            VersionString::new("7"),
            VersionString::new("9"),
            VersionString::new("10"),
        ],
    );

    assert_eq!(
        result,
        AST::new(vec![
            Line::ToolDefinition {
                name: Identifier::new("nodejs"),
                versions: Versions::new(vec![
                    Version::new(VersionString::new("7"), Whitespace::new("  ")),
                    Version::new(VersionString::new("9"), Whitespace::new("    ")),
                    Version::new(VersionString::new("10"), Whitespace::new(" ")),
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

    let result = transformer::set_versions(
        &ast,
        Identifier::new("ruby"),
        vec![VersionString::new("14")],
    );

    assert_eq!(
        result,
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
                versions: Versions::new(vec![Version::new(
                    VersionString::new("14"),
                    Whitespace::new("    ")
                ),]),
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
        vec![VersionString::new("1337")],
    );

    assert_eq!(
        result,
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
                error: SyntaxError::DuplicateIdentifier("lua".to_string()),
                unparsed: Unparsed::new("lua   "),
            },
            Line::Invalid {
                error: SyntaxError::UnexpectedEOL {
                    expected: "Version"
                },
                unparsed: Unparsed::new("golang "),
            },
            Line::ToolDefinition {
                name: Identifier::new("golang"),
                versions: Versions::new(vec![Version::new(
                    VersionString::new("1337"),
                    Whitespace::new(" ")
                ),]),
                whitespace: None,
                comment: None
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
        vec![VersionString::new("1337")],
    );

    assert_eq!(
        result,
        AST::new(vec![Line::ToolDefinition {
            name: Identifier::new("golang"),
            versions: Versions::new(vec![Version::new(
                VersionString::new("1337"),
                Whitespace::new(" ")
            ),]),
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
            Line::ToolDefinition {
                name: Identifier::new("nodejs"),
                versions: Versions::new(vec![
                    Version::new(VersionString::new("18.12"), Whitespace::new("  ")),
                    Version::new(VersionString::new("system"), Whitespace::new("    ")),
                ]),
                whitespace: Some(Whitespace::new("  ")),
                comment: Some(Unparsed::new(" foobar  "))
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
