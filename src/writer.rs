use crate::ast::{Line, Tool, AST};
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
            Line::Definition {
                tool,
                whitespace,
                comment,
            } => {
                let mut s = String::from(tool.to_string());

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

impl Tool {
    fn to_string(&self) -> String {
        let mut s = String::from(&self.name.0);

        let versions: Vec<String> = self
            .versions
            .iter()
            .map(|version| {
                let mut s = String::from(&version.left_padding.0);
                s.push_str(&version.value.0);
                s
            })
            .collect();

        s.push_str(&versions.join(""));

        s
    }
}
