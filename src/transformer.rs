use crate::ast::{Identifier, Line, Node, Version, Versions, Whitespace, AST};

pub fn set_versions(ast: &AST, tool_name: Identifier, versions: Vec<Version>) -> AST {
    let versions: Vec<(Whitespace, Version)> = versions
        .iter()
        .map(|version| (Whitespace::new(" ".to_string()), version.clone()))
        .collect();

    if ast.lines.len() == 0 {
        if versions.len() == 0 {
            return AST { lines: vec![] };
        }

        return AST {
            lines: vec![Line::ToolDefinition {
                name: tool_name,
                versions: Versions::new(versions),
                whitespace: None,
                comment: None,
            }],
        };
    }

    if !ast.lines.iter().any(|line| match line {
        Line::ToolDefinition { name, .. } if *name == tool_name => true,
        _ => false,
    }) {
        let mut lines = ast.lines.clone();

        if versions.len() == 0 {
            return AST { lines };
        }

        lines.push(Line::ToolDefinition {
            name: tool_name,
            versions: Versions::new(versions),
            whitespace: None,
            comment: None,
        });

        return AST { lines };
    }

    AST {
        lines: ast
            .lines
            .iter()
            .filter_map(|line| match line {
                Line::ToolDefinition {
                    name,
                    whitespace,
                    comment,
                    versions: old_versions,
                } if *name == tool_name => {
                    if versions.len() == 0 {
                        return None;
                    }

                    let new_versions = versions
                        .iter()
                        .enumerate()
                        .map(|(i, (_, version))| {
                            let left_padding =
                                if let Some((old_padding, _)) = old_versions.value().get(i) {
                                    old_padding.clone()
                                } else {
                                    Whitespace::new(" ".to_string())
                                };

                            (left_padding, version.clone())
                        })
                        .collect();

                    Some(Line::ToolDefinition {
                        name: name.clone(),
                        versions: Versions::new(new_versions),
                        whitespace: whitespace.clone(),
                        comment: comment.clone(),
                    })
                }
                line => Some(line.clone()),
            })
            .collect(),
    }
}
