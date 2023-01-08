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

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier(pub(crate) String);

#[derive(Debug, PartialEq, Clone)]
pub struct VersionString(pub(crate) String);

#[derive(Debug, PartialEq, Clone)]
pub struct Whitespace(pub(crate) String);

#[derive(Debug, PartialEq, Clone)]
pub struct Unparsed(pub(crate) String);

#[derive(Debug, PartialEq, Clone)]
pub enum SyntaxError {
    UnexpectedToken { token: char, expected: &'static str },
    UnexpectedEOL { expected: &'static str },
    DuplicateIdentifier(String),
}

pub trait Token {
    fn is_valid_char(c: char) -> bool;
}

impl AST {
    pub fn new(lines: Vec<Line>) -> Self {
        AST(lines)
    }
}

#[derive(Debug)]
pub enum VersionsConstructorError {
    MissingVersions,
}

impl Versions {
    pub fn new(versions: Vec<Version>) -> Self {
        match Self::try_new(versions) {
            Ok(tool) => tool,
            Err(e) => panic!("{:?}", e),
        }
    }

    pub fn try_new(versions: Vec<Version>) -> Result<Self, VersionsConstructorError> {
        if versions.len() == 0 {
            return Err(VersionsConstructorError::MissingVersions);
        }

        Ok(Self(versions))
    }
}

#[derive(Debug)]
pub enum VersionConstructorError {
    InvalidLeftPadding(Whitespace),
}

impl Version {
    pub fn new(value: VersionString, left_padding: Whitespace) -> Self {
        match Self::try_new(value, left_padding) {
            Ok(version) => version,
            Err(e) => panic!("{:?}", e),
        }
    }

    pub fn try_new(
        value: VersionString,
        left_padding: Whitespace,
    ) -> Result<Self, VersionConstructorError> {
        if left_padding.0.len() == 0 {
            return Err(VersionConstructorError::InvalidLeftPadding(left_padding));
        }

        Ok(Self {
            value,
            left_padding,
        })
    }
}

impl Token for Identifier {
    fn is_valid_char(c: char) -> bool {
        matches!(c, '0'..='9' | 'A'..='Z' | 'a'..='z' | '.'| '-'| '_')
    }
}

#[derive(Debug)]
pub enum IdentifierConstructorError {
    InvalidValue(String),
}

impl Identifier {
    pub fn new(value: &str) -> Self {
        match Self::try_new(value) {
            Ok(identifier) => identifier,
            Err(e) => panic!("{:?}", e),
        }
    }

    pub fn try_new(value: &str) -> Result<Self, IdentifierConstructorError> {
        if !value.chars().all(Self::is_valid_char) {
            return Err(IdentifierConstructorError::InvalidValue(value.to_string()));
        }

        Ok(Self(value.to_string()))
    }
}

impl Token for VersionString {
    fn is_valid_char(c: char) -> bool {
        !c.is_whitespace() && c != '#'
    }
}

#[derive(Debug)]
pub enum StringValueConstructorError {
    InvalidValue(String),
}

impl VersionString {
    pub fn new(value: &str) -> Self {
        match Self::try_new(value) {
            Ok(string_value) => string_value,
            Err(e) => panic!("{:?}", e),
        }
    }

    pub fn try_new(value: &str) -> Result<Self, StringValueConstructorError> {
        if !value.chars().all(Self::is_valid_char) {
            return Err(StringValueConstructorError::InvalidValue(value.to_string()));
        }

        Ok(Self(value.to_string()))
    }
}

impl Token for Whitespace {
    fn is_valid_char(c: char) -> bool {
        c.is_whitespace() && c != '\n' && c != '\r'
    }
}

#[derive(Debug)]
pub enum WhitespaceConstructorError {
    InvalidValue(String),
}

impl Whitespace {
    pub fn new(value: &str) -> Self {
        match Self::try_new(value) {
            Ok(whitespace) => whitespace,
            Err(e) => panic!("{:?}", e),
        }
    }

    pub fn try_new(value: &str) -> Result<Self, WhitespaceConstructorError> {
        if !value.chars().all(Self::is_valid_char) {
            return Err(WhitespaceConstructorError::InvalidValue(value.to_string()));
        }

        Ok(Self(value.to_string()))
    }
}

#[derive(Debug)]
pub enum UnparsedConstructorError {
    InvalidValue(String),
}

impl Unparsed {
    pub fn new(value: &str) -> Self {
        match Self::try_new(value) {
            Ok(unparsed) => unparsed,
            Err(e) => panic!("{:?}", e),
        }
    }

    pub fn try_new(value: &str) -> Result<Self, UnparsedConstructorError> {
        if value.chars().any(|c| c == '\n' || c == '\r') {
            return Err(UnparsedConstructorError::InvalidValue(value.to_string()));
        }

        Ok(Self(value.to_string()))
    }
}
