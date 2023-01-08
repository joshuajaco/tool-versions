use crate::ast::{Identifier, Line, Version, VersionString, Versions, Whitespace, AST};

pub fn set_versions(ast: &AST, tool_name: Identifier, versions: Vec<VersionString>) -> AST {
    let versions: Vec<Version> = versions
        .iter()
        .map(|version| Version {
            value: version.clone(),
            left_padding: Whitespace::from(" ".to_string()),
        })
        .collect();

    if ast.0.len() == 0 {
        if versions.len() == 0 {
            return AST(vec![]);
        }

        return AST(vec![Line::ToolDefinition {
            name: tool_name,
            versions: Versions(versions),
            whitespace: None,
            comment: None,
        }]);
    }

    if !ast.0.iter().any(|line| match line {
        Line::ToolDefinition { name, .. } if *name == tool_name => true,
        _ => false,
    }) {
        let mut lines = ast.0.clone();

        if versions.len() == 0 {
            return AST(lines);
        }

        lines.push(Line::ToolDefinition {
            name: tool_name,
            versions: Versions(versions),
            whitespace: None,
            comment: None,
        });

        return AST(lines);
    }

    AST(ast
        .0
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
                    .map(|(i, version)| {
                        let value = version.value.clone();

                        let left_padding = if let Some(old_version) = old_versions.0.get(i) {
                            old_version.left_padding.clone()
                        } else {
                            Whitespace::from(" ".to_string())
                        };

                        Version {
                            value,
                            left_padding,
                        }
                    })
                    .collect();

                Some(Line::ToolDefinition {
                    name: name.clone(),
                    versions: Versions(new_versions),
                    whitespace: whitespace.clone(),
                    comment: comment.clone(),
                })
            }
            line => Some(line.clone()),
        })
        .collect())
}
