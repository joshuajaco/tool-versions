use crate::ast::{Identifier, Line, SyntaxError, Tool, Unparsed, Version, Whitespace, AST};
use std::{fs, io, path::Path, str::Chars};

pub fn parse_file<P: AsRef<Path>>(path: P) -> io::Result<AST> {
    let input = fs::read_to_string(path)?;
    Ok(parse(&input))
}

pub fn parse(input: &str) -> AST {
    AST(input.lines().map(parse_line).collect())
}

fn parse_line(line: &str) -> Line {
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

            match take_whitespace(&mut chars, &mut whitespace) {
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
        Some(first) => parse_definition(line, first, &mut chars),
    }
}

fn parse_definition(line: &str, first: char, chars: &mut Chars) -> Line {
    let mut tool_name = String::from(first);

    match take_identifier(chars, &mut tool_name) {
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
        Some(next) => {
            let name = Identifier(tool_name);
            let mut versions: Vec<Version> = vec![];
            let mut whitespace = String::from(next);

            loop {
                match take_whitespace(chars, &mut whitespace) {
                    None if versions.len() == 0 => {
                        return Line::Invalid {
                            error: SyntaxError::UnexpectedEOL {
                                expected: "Version",
                            },
                            unparsed: Unparsed(line.to_string()),
                        }
                    }
                    None => {
                        return Line::Definition {
                            tool: Tool { name, versions },
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
                        return Line::Definition {
                            tool: Tool { name, versions },
                            whitespace: Some(Whitespace(whitespace)),
                            comment: Some(Unparsed(chars.collect())),
                        }
                    }
                    Some(next) => {
                        let mut version = String::from(next);
                        match take_identifier(chars, &mut version) {
                            None => {
                                versions.push(Version {
                                    value: Identifier(version),
                                    left_padding: Whitespace(whitespace),
                                });

                                return Line::Definition {
                                    tool: Tool { name, versions },
                                    whitespace: None,
                                    comment: None,
                                };
                            }
                            Some(next) => {
                                versions.push(Version {
                                    value: Identifier(version),
                                    left_padding: Whitespace(whitespace),
                                });

                                if next == '#' {
                                    return Line::Definition {
                                        tool: Tool { name, versions },
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

fn take_whitespace(chars: &mut Chars, whitespace: &mut String) -> Option<char> {
    take_until(chars, whitespace, |c| !c.is_whitespace())
}

fn take_identifier(chars: &mut Chars, whitespace: &mut String) -> Option<char> {
    take_until(chars, whitespace, |c| c.is_whitespace() || c == '#')
}

fn take_until(chars: &mut Chars, output: &mut String, cb: fn(char) -> bool) -> Option<char> {
    loop {
        match chars.next() {
            None => return None,
            Some(next) => {
                if cb(next) {
                    return Some(next);
                } else {
                    output.push(next);
                }
            }
        }
    }
}
