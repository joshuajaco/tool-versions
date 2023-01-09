use crate::ast::Node;
use std::{io, path::Path};

pub mod ast;
pub mod parser;
pub mod transformer;
pub mod writer;

pub struct ToolVersions {
    ast: ast::AST,
}

impl ToolVersions {
    pub fn new() -> Self {
        ToolVersions {
            ast: ast::AST { lines: vec![] },
        }
    }

    pub fn from(s: &str) -> Self {
        ToolVersions {
            ast: parser::parse(s),
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let ast = parser::parse_file(path)?;
        Ok(ToolVersions { ast })
    }

    pub fn errors(&self) -> Vec<&ast::SyntaxError> {
        self.ast
            .lines
            .iter()
            .filter_map(|line| match line {
                ast::Line::Invalid { error, .. } => Some(error),
                _ => None,
            })
            .collect()
    }

    pub fn versions(&self, tool_name: &str) -> Option<Vec<String>> {
        self.ast.lines.iter().find_map(|line| match line {
            ast::Line::ToolDefinition { name, versions, .. } if name.value() == tool_name => Some(
                versions
                    .value()
                    .iter()
                    .map(|(_, v)| v.value().clone())
                    .collect(),
            ),
            _ => None,
        })
    }

    pub fn set_versions(&mut self, tool_name: &str, versions: Vec<&str>) {
        self.ast = transformer::set_versions(
            &self.ast,
            ast::Identifier::new(tool_name.to_string()),
            versions
                .iter()
                .map(|version| ast::Version::new(version.to_string()))
                .collect(),
        );
    }

    pub fn write(&self) -> String {
        writer::write(&self.ast)
    }

    pub fn write_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        writer::write_file(&self.ast, path)
    }
}
