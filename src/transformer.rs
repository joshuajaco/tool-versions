use crate::ast::{Identifier, Line, Tool, Version, Whitespace, AST};

pub fn set_versions(ast: &AST, tool_name: &str, versions: Vec<&str>) -> AST {
    AST(ast
        .0
        .iter()
        .filter_map(|line| match line {
            Line::Definition {
                tool,
                whitespace,
                comment,
            } if tool.name.0.eq(tool_name) => {
                if versions.len() == 0 {
                    return None;
                }

                let new_versions = versions
                    .iter()
                    .enumerate()
                    .map(|(i, version)| {
                        let value = Identifier(version.to_string());

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
                        name: tool.name.clone(),
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
