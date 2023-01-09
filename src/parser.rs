use crate::ast::{
    Identifier, Line, Node, SyntaxError, Unparsed, Version, Versions, Whitespace, AST,
};
use std::{collections::HashSet, fs, io, path::Path, str::Chars};

pub fn parse_file<P: AsRef<Path>>(path: P) -> io::Result<AST> {
    let input = fs::read_to_string(path)?;
    Ok(parse(&input))
}

pub fn parse(input: &str) -> AST {
    let mut unique_identifiers = HashSet::new();

    AST {
        lines: input
            .lines()
            .map(|line| parse_line(line, &mut unique_identifiers))
            .collect(),
    }
}

fn parse_line(line: &str, unique_identifiers: &mut HashSet<Identifier>) -> Line {
    let mut chars = line.chars();

    match chars.next() {
        Some(first) if Identifier::is_consumable(first) => {
            parse_definition(line, first, &mut chars, unique_identifiers)
        }
        Some(first) if Whitespace::is_consumable(first) => {
            let (whitespace, next) = consume::<Whitespace>(first, &mut chars);

            match next {
                None => Line::Empty {
                    whitespace: Some(whitespace),
                    comment: None,
                },
                Some('#') => Line::Empty {
                    whitespace: Some(whitespace),
                    comment: Some(Unparsed::new(chars.collect())),
                },
                Some(token) => Line::Invalid {
                    error: SyntaxError::UnexpectedToken {
                        token,
                        expected: "EOL,Comment",
                    },
                    unparsed: Unparsed::new(line.to_string()),
                },
            }
        }
        Some('#') => Line::Empty {
            whitespace: None,
            comment: Some(Unparsed::new(chars.collect())),
        },
        None => Line::Empty {
            whitespace: None,
            comment: None,
        },
        Some(token) => Line::Invalid {
            error: SyntaxError::UnexpectedToken {
                token,
                expected: "Identifier,Whitespace,Comment",
            },
            unparsed: Unparsed::new(line.to_string()),
        },
    }
}

fn parse_definition(
    line: &str,
    first: char,
    chars: &mut Chars,
    unique_identifiers: &mut HashSet<Identifier>,
) -> Line {
    let (name, next) = consume::<Identifier>(first, chars);

    match next {
        None => Line::Invalid {
            error: SyntaxError::UnexpectedEOL {
                expected: "Whitespace",
            },
            unparsed: Unparsed::new(line.to_string()),
        },
        Some(token) if !Whitespace::is_consumable(token) => Line::Invalid {
            error: SyntaxError::UnexpectedToken {
                token,
                expected: "Whitespace",
            },
            unparsed: Unparsed::new(line.to_string()),
        },
        Some(next) => {
            if unique_identifiers.contains(&name) {
                return Line::Invalid {
                    error: SyntaxError::DuplicateIdentifier(name),
                    unparsed: Unparsed::new(line.to_string()),
                };
            }

            unique_identifiers.insert(name.clone());

            let mut versions = Vec::new();
            let mut first = next;

            loop {
                let (whitespace, next) = consume::<Whitespace>(first, chars);

                match next {
                    None if versions.len() == 0 => {
                        return Line::Invalid {
                            error: SyntaxError::UnexpectedEOL {
                                expected: "Version",
                            },
                            unparsed: Unparsed::new(line.to_string()),
                        }
                    }
                    None => {
                        return Line::ToolDefinition {
                            name,
                            versions: Versions::new(versions),
                            whitespace: Some(whitespace),
                            comment: None,
                        }
                    }
                    Some('#') if versions.len() == 0 => {
                        return Line::Invalid {
                            error: SyntaxError::UnexpectedToken {
                                token: '#',
                                expected: "Version",
                            },
                            unparsed: Unparsed::new(line.to_string()),
                        }
                    }
                    Some('#') => {
                        return Line::ToolDefinition {
                            name,
                            versions: Versions::new(versions),
                            whitespace: Some(whitespace),
                            comment: Some(Unparsed::new(chars.collect())),
                        }
                    }
                    Some(next) => {
                        let (version, next) = consume::<Version>(next, chars);

                        match next {
                            None => {
                                versions.push((whitespace, version));

                                return Line::ToolDefinition {
                                    name,
                                    versions: Versions::new(versions),
                                    whitespace: None,
                                    comment: None,
                                };
                            }
                            Some(next) => {
                                versions.push((whitespace, version));

                                if next == '#' {
                                    return Line::ToolDefinition {
                                        name,
                                        versions: Versions::new(versions),
                                        whitespace: None,
                                        comment: Some(Unparsed::new(chars.collect())),
                                    };
                                }

                                first = next;
                            }
                        }
                    }
                }
            }
        }
    }
}

trait Consumable: Node<String> {
    fn is_consumable(c: char) -> bool;
}

impl Consumable for Identifier {
    fn is_consumable(c: char) -> bool {
        matches!(c, '0'..='9' | 'A'..='Z' | 'a'..='z' | '.'| '-'| '_')
    }
}

impl Consumable for Version {
    fn is_consumable(c: char) -> bool {
        !c.is_whitespace() && c != '#'
    }
}

impl Consumable for Whitespace {
    fn is_consumable(c: char) -> bool {
        c.is_whitespace()
    }
}

fn consume<T: Consumable>(first: char, chars: &mut Chars) -> (T, Option<char>) {
    let mut output = String::from(first);

    loop {
        match chars.next() {
            Some(next) if T::is_consumable(next) => {
                output.push(next);
            }
            next => return (T::new(output), next),
        }
    }
}
