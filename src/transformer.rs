use crate::ast::{Identifier, Line, Tool, Version, Whitespace, AST};

pub fn set_versions(ast: &AST, name: Identifier, versions: Vec<Identifier>) -> AST {
    let versions: Vec<Version> = versions
        .iter()
        .map(|version| Version {
            value: version.clone(),
            left_padding: Whitespace(" ".to_string()),
        })
        .collect();

    if ast.0.len() == 0 {
        if versions.len() == 0 {
            return AST(vec![]);
        }

        return AST(vec![Line::Definition {
            tool: Tool { name, versions },
            whitespace: None,
            comment: None,
        }]);
    }

    if !ast.0.iter().any(|line| match line {
        Line::Definition { tool, .. } if tool.name == name => true,
        _ => false,
    }) {
        let mut lines = ast.0.clone();

        if versions.len() == 0 {
            return AST(lines);
        }

        lines.push(Line::Definition {
            tool: Tool { name, versions },
            whitespace: None,
            comment: None,
        });

        return AST(lines);
    }

    AST(ast
        .0
        .iter()
        .filter_map(|line| match line {
            Line::Definition {
                tool,
                whitespace,
                comment,
            } if tool.name == name => {
                if versions.len() == 0 {
                    return None;
                }

                let new_versions = versions
                    .iter()
                    .enumerate()
                    .map(|(i, version)| {
                        let value = version.value.clone();

                        let left_padding = if let Some(old_version) = tool.versions.get(i) {
                            old_version.left_padding.clone()
                        } else {
                            Whitespace(" ".to_string())
                        };

                        Version {
                            value,
                            left_padding,
                        }
                    })
                    .collect();

                Some(Line::Definition {
                    tool: Tool {
                        name: name.clone(),
                        versions: new_versions,
                    },
                    whitespace: whitespace.clone(),
                    comment: comment.clone(),
                })
            }
            line => Some(line.clone()),
        })
        .collect())
}
