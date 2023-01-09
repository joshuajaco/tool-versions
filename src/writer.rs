use crate::ast::{Line, Node, AST};
use std::{fs, io, path::Path};

pub fn write_file<P: AsRef<Path>>(ast: &AST, path: P) -> io::Result<()> {
    fs::write(path, write(ast))
}

pub fn write(ast: &AST) -> String {
    let lines: Vec<String> = ast.lines.iter().map(|line| line.to_string()).collect();

    let mut result = String::from(lines.join("\n"));

    result.push_str("\n");

    result
}

impl Line {
    fn to_string(&self) -> String {
        match self {
            Line::ToolDefinition {
                name,
                versions,
                whitespace,
                comment,
            } => {
                let mut s = String::from(name.value());

                s.push_str(
                    &versions
                        .value()
                        .iter()
                        .map(|(left_padding, version)| {
                            let mut s = String::from(left_padding.value());
                            s.push_str(version.value());
                            s
                        })
                        .collect::<Vec<String>>()
                        .join(""),
                );

                if let Some(whitespace) = whitespace {
                    s.push_str(whitespace.value());
                }

                if let Some(comment) = comment {
                    s.push_str("#");
                    s.push_str(comment.value());
                }

                s
            }
            Line::Empty {
                whitespace,
                comment,
            } => {
                let mut s = String::new();

                if let Some(whitespace) = whitespace {
                    s.push_str(whitespace.value());
                }

                if let Some(comment) = comment {
                    s.push_str("#");
                    s.push_str(comment.value());
                }

                s
            }
            Line::Invalid { unparsed, .. } => unparsed.value().clone(),
        }
    }
}
