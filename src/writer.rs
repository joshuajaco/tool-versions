use crate::ast::{Line, AST};
use std::{fs, io, path::Path};

pub fn write_file<P: AsRef<Path>>(ast: &AST, path: P) -> io::Result<()> {
    fs::write(path, write(ast))
}

pub fn write(ast: &AST) -> String {
    let lines: Vec<String> = ast.0.iter().map(|line| line.to_string()).collect();

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
                let mut s = String::from(&name.0);

                s.push_str(
                    &versions
                        .0
                        .iter()
                        .map(|version| {
                            let mut s = String::from(&version.left_padding.0);
                            s.push_str(&version.value.0);
                            s
                        })
                        .collect::<Vec<String>>()
                        .join(""),
                );

                if let Some(whitespace) = whitespace {
                    s.push_str(&whitespace.0);
                }

                if let Some(comment) = comment {
                    s.push_str("#");
                    s.push_str(&comment.0);
                }

                s
            }
            Line::Empty {
                whitespace,
                comment,
            } => {
                let mut s = String::new();

                if let Some(whitespace) = whitespace {
                    s.push_str(&whitespace.0);
                }

                if let Some(comment) = comment {
                    s.push_str("#");
                    s.push_str(&comment.0);
                }

                s
            }
            Line::Invalid { unparsed, .. } => unparsed.0.clone(),
        }
    }
}
