use crate::ast::{
    Identifier, Line, SyntaxError, Token, Unparsed, Version, VersionString, Versions, Whitespace,
    AST,
};

use std::{collections::HashSet, fs, io, path::Path, str::Chars};

pub fn parse_file<P: AsRef<Path>>(path: P) -> io::Result<AST> {
    let input = fs::read_to_string(path)?;
    Ok(parse(&input))
}

pub fn parse(input: &str) -> AST {
    let mut unique_identifiers = HashSet::new();

    AST(input
        .lines()
        .map(|line| parse_line(line, &mut unique_identifiers))
        .collect())
}

fn parse_line(line: &str, unique_identifiers: &mut HashSet<String>) -> Line {
    let mut chars = line.chars();

    match chars.next() {
        None => Line::Empty {
            whitespace: None,
            comment: None,
        },
        Some('#') => Line::Empty {
            whitespace: None,
            comment: Some(Unparsed(chars.collect())),
        },
        Some(first) if first.is_whitespace() => {
            let mut whitespace = String::from(first);

            match take::<Whitespace>(&mut chars, &mut whitespace) {
                None => Line::Empty {
                    whitespace: Some(Whitespace(whitespace)),
                    comment: None,
                },
                Some('#') => Line::Empty {
                    whitespace: Some(Whitespace(whitespace)),
                    comment: Some(Unparsed(chars.collect())),
                },
                Some(token) => Line::Invalid {
                    error: SyntaxError::UnexpectedToken {
                        token,
                        expected: "EOL,Comment",
                    },
                    unparsed: Unparsed(line.to_string()),
                },
            }
        }
        Some(first) => parse_definition(line, first, &mut chars, unique_identifiers),
    }
}

fn parse_definition(
    line: &str,
    first: char,
    chars: &mut Chars,
    unique_identifiers: &mut HashSet<String>,
) -> Line {
    let mut tool_name = String::from(first);

    match take::<Identifier>(chars, &mut tool_name) {
        None => Line::Invalid {
            error: SyntaxError::UnexpectedEOL {
                expected: "Version",
            },
            unparsed: Unparsed(line.to_string()),
        },
        Some('#') => Line::Invalid {
            error: SyntaxError::UnexpectedToken {
                token: '#',
                expected: "Version",
            },
            unparsed: Unparsed(line.to_string()),
        },
        Some(token) if !token.is_whitespace() => Line::Invalid {
            error: SyntaxError::UnexpectedToken {
                token,
                expected: "Whitespace",
            },
            unparsed: Unparsed(line.to_string()),
        },
        Some(next) => {
            if unique_identifiers.contains(&tool_name) {
                return Line::Invalid {
                    error: SyntaxError::DuplicateIdentifier(tool_name),
                    unparsed: Unparsed(line.to_string()),
                };
            }

            unique_identifiers.insert(tool_name.clone());

            let name = Identifier(tool_name);
            let mut versions = Vec::new();
            let mut whitespace = String::from(next);

            loop {
                match take::<Whitespace>(chars, &mut whitespace) {
                    None if versions.len() == 0 => {
                        return Line::Invalid {
                            error: SyntaxError::UnexpectedEOL {
                                expected: "Version",
                            },
                            unparsed: Unparsed(line.to_string()),
                        }
                    }
                    None => {
                        return Line::ToolDefinition {
                            name,
                            versions: Versions(versions),
                            whitespace: Some(Whitespace(whitespace)),
                            comment: None,
                        }
                    }
                    Some('#') if versions.len() == 0 => {
                        return Line::Invalid {
                            error: SyntaxError::UnexpectedToken {
                                token: '#',
                                expected: "Version",
                            },
                            unparsed: Unparsed(line.to_string()),
                        }
                    }
                    Some('#') => {
                        return Line::ToolDefinition {
                            name,
                            versions: Versions(versions),
                            whitespace: Some(Whitespace(whitespace)),
                            comment: Some(Unparsed(chars.collect())),
                        }
                    }
                    Some(next) => {
                        let mut version = String::from(next);

                        match take::<VersionString>(chars, &mut version) {
                            None => {
                                versions.push(Version {
                                    value: VersionString(version),
                                    left_padding: Whitespace(whitespace),
                                });

                                return Line::ToolDefinition {
                                    name,
                                    versions: Versions(versions),
                                    whitespace: None,
                                    comment: None,
                                };
                            }
                            Some(next) => {
                                versions.push(Version {
                                    value: VersionString(version),
                                    left_padding: Whitespace(whitespace),
                                });

                                if next == '#' {
                                    return Line::ToolDefinition {
                                        name,
                                        versions: Versions(versions),
                                        whitespace: None,
                                        comment: Some(Unparsed(chars.collect())),
                                    };
                                }

                                whitespace = String::from(next);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn take<T>(chars: &mut Chars, output: &mut String) -> Option<char>
where
    T: Token,
{
    loop {
        match chars.next() {
            None => return None,
            Some(next) => {
                if !T::is_valid_char(next) {
                    return Some(next);
                } else {
                    output.push(next);
                }
            }
        }
    }
}
