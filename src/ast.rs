#[derive(Debug, PartialEq)]
pub struct AST {
    pub lines: Vec<Line>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Line {
    ToolDefinition {
        name: Identifier,
        versions: Versions,
        whitespace: Option<Whitespace>,
        comment: Option<Unparsed>,
    },
    Empty {
        whitespace: Option<Whitespace>,
        comment: Option<Unparsed>,
    },
    Invalid {
        error: SyntaxError,
        unparsed: Unparsed,
    },
}

pub trait Node<T> {
    fn new(value: T) -> Self;
    fn value(&self) -> &T;
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Identifier(String);

impl Node<String> for Identifier {
    fn new(value: String) -> Self {
        Self(value)
    }

    fn value(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Versions(Vec<(Whitespace, Version)>);

impl Node<Vec<(Whitespace, Version)>> for Versions {
    fn new(value: Vec<(Whitespace, Version)>) -> Self {
        Self(value)
    }

    fn value(&self) -> &Vec<(Whitespace, Version)> {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Version(String);

impl Node<String> for Version {
    fn new(value: String) -> Self {
        Self(value)
    }

    fn value(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Whitespace(String);

impl Node<String> for Whitespace {
    fn new(value: String) -> Self {
        Self(value)
    }

    fn value(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Unparsed(String);

impl Node<String> for Unparsed {
    fn new(value: String) -> Self {
        Self(value)
    }

    fn value(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SyntaxError {
    UnexpectedToken { token: char, expected: &'static str },
    UnexpectedEOL { expected: &'static str },
    DuplicateIdentifier(Identifier),
}
