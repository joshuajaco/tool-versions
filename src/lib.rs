use std::{io, path::Path};

pub mod ast;
pub mod parser;
pub mod transformer;
pub mod writer;

pub struct ToolVersions {
    ast: ast::AST,
}

impl ToolVersions {
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
            .0
            .iter()
            .filter_map(|line| match line {
                ast::Line::Invalid { error, .. } => Some(error),
                _ => None,
            })
            .collect()
    }

    pub fn versions(&self, tool_name: &str) -> Option<Vec<String>> {
        self.ast
            .0
            .iter()
            .filter_map(|line| match line {
                ast::Line::Definition { tool, .. } => Some(tool),
                _ => None,
            })
            .find(|tool| tool.name.0.eq(tool_name))
            .map(|tool| tool.versions.iter().map(|v| v.value.0.clone()).collect())
    }

    pub fn set_versions(&mut self, tool_name: &str, versions: Vec<&str>) {
        self.ast = transformer::set_versions(
            &self.ast,
            ast::Identifier::new(tool_name),
            versions
                .iter()
                .map(|version| ast::Identifier::new(version))
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
