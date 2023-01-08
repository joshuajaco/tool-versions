#[derive(Debug, PartialEq)]
pub struct AST(pub(crate) Vec<Line>);

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

#[derive(Debug, PartialEq, Clone)]
pub struct Versions(pub(crate) Vec<Version>);

#[derive(Debug, PartialEq, Clone)]
pub struct Version {
    pub(crate) value: VersionString,
    pub(crate) left_padding: Whitespace,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Identifier(String);

#[derive(Debug, PartialEq, Clone)]
pub struct VersionString(String);

#[derive(Debug, PartialEq, Clone)]
pub struct Whitespace(String);

#[derive(Debug, PartialEq, Clone)]
pub struct Unparsed(String);

#[derive(Debug, PartialEq, Clone)]
pub enum SyntaxError {
    UnexpectedToken { token: char, expected: &'static str },
    UnexpectedEOL { expected: &'static str },
    DuplicateIdentifier(Identifier),
}

pub trait Token: From<String> {
    fn value(&self) -> &String;
}

impl AST {
    pub fn new(lines: Vec<Line>) -> Self {
        AST(lines)
    }
}

impl Versions {
    pub fn new(versions: Vec<Version>) -> Self {
        match Self::try_new(versions) {
            Some(tool) => tool,
            None => panic!("invalid Versions"),
        }
    }

    pub fn try_new(versions: Vec<Version>) -> Option<Self> {
        if versions.len() == 0 {
            return None;
        }

        Some(Self(versions))
    }
}

impl Version {
    pub fn new(value: VersionString, left_padding: Whitespace) -> Self {
        match Self::try_new(value, left_padding) {
            Some(version) => version,
            None => panic!("invalid Version"),
        }
    }

    pub fn try_new(value: VersionString, left_padding: Whitespace) -> Option<Self> {
        if left_padding.0.len() == 0 {
            return None;
        }

        Some(Self {
            value,
            left_padding,
        })
    }
}

impl Identifier {
    pub fn new(value: &str) -> Self {
        match Self::try_new(value) {
            Some(identifier) => identifier,
            None => panic!("invalid Identifier"),
        }
    }

    pub fn try_new(value: &str) -> Option<Self> {
        if !value
            .chars()
            .all(|c| matches!(c, '0'..='9' | 'A'..='Z' | 'a'..='z' | '.'| '-'| '_'))
        {
            return None;
        }

        Some(Self(value.to_string()))
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Token for Identifier {
    fn value(&self) -> &String {
        &self.0
    }
}

impl VersionString {
    pub fn new(value: &str) -> Self {
        match Self::try_new(value) {
            Some(string_value) => string_value,
            None => panic!("invalid VersionString"),
        }
    }

    pub fn try_new(value: &str) -> Option<Self> {
        if !value.chars().all(|c| !c.is_whitespace() && c != '#') {
            return None;
        }

        Some(Self(value.to_string()))
    }
}

impl From<String> for VersionString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Token for VersionString {
    fn value(&self) -> &String {
        &self.0
    }
}

impl Whitespace {
    pub fn new(value: &str) -> Self {
        match Self::try_new(value) {
            Some(whitespace) => whitespace,
            None => panic!("invalid Whitespace"),
        }
    }

    pub fn try_new(value: &str) -> Option<Self> {
        if !value
            .chars()
            .all(|c| c.is_whitespace() && c != '\n' && c != '\r')
        {
            return None;
        }

        Some(Self(value.to_string()))
    }
}

impl From<String> for Whitespace {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Token for Whitespace {
    fn value(&self) -> &String {
        &self.0
    }
}

impl Unparsed {
    pub fn new(value: &str) -> Self {
        match Self::try_new(value) {
            Some(unparsed) => unparsed,
            None => panic!("invalid Unparsed"),
        }
    }

    pub fn try_new(value: &str) -> Option<Self> {
        if value.chars().any(|c| c == '\n' || c == '\r') {
            return None;
        }

        Some(Self(value.to_string()))
    }
}

impl From<String> for Unparsed {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Token for Unparsed {
    fn value(&self) -> &String {
        &self.0
    }
}
