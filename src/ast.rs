#[derive(Debug, PartialEq)]
pub struct AST(pub(crate) Vec<Line>);

#[derive(Debug, PartialEq, Clone)]
pub enum Line {
    Definition {
        tool: Tool,
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
pub struct Tool {
    pub(crate) name: Identifier,
    pub(crate) versions: Vec<Version>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Version {
    pub(crate) value: StringValue,
    pub(crate) left_padding: Whitespace,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier(pub(crate) String);

#[derive(Debug, PartialEq, Clone)]
pub struct StringValue(pub(crate) String);

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

impl AST {
    pub fn new(lines: Vec<Line>) -> Self {
        AST(lines)
    }
}

#[derive(Debug)]
pub enum ToolConstructorError {
    InvalidName(String),
    MissingVersions,
}

impl Tool {
    pub fn new(name: Identifier, versions: Vec<Version>) -> Self {
        match Self::try_new(name, versions) {
            Ok(tool) => tool,
            Err(e) => panic!("{:?}", e),
        }
    }

    pub fn try_new(name: Identifier, versions: Vec<Version>) -> Result<Self, ToolConstructorError> {
        if versions.len() == 0 {
            return Err(ToolConstructorError::MissingVersions);
        }

        Ok(Self { name, versions })
    }
}

#[derive(Debug)]
pub enum VersionConstructorError {
    InvalidLeftPadding(Whitespace),
}

impl Version {
    pub fn new(value: StringValue, left_padding: Whitespace) -> Self {
        match Self::try_new(value, left_padding) {
            Ok(version) => version,
            Err(e) => panic!("{:?}", e),
        }
    }

    pub fn try_new(
        value: StringValue,
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
        if !value
            .chars()
            .all(|c| matches!(c, '0'..='9' | 'A'..='Z' | 'a'..='z' | '.'| '-'| '_'))
        {
            return Err(IdentifierConstructorError::InvalidValue(value.to_string()));
        }

        Ok(Self(value.to_string()))
    }
}

#[derive(Debug)]
pub enum StringValueConstructorError {
    InvalidValue(String),
}

impl StringValue {
    pub fn new(value: &str) -> Self {
        match Self::try_new(value) {
            Ok(string_value) => string_value,
            Err(e) => panic!("{:?}", e),
        }
    }

    pub fn try_new(value: &str) -> Result<Self, StringValueConstructorError> {
        if value.chars().any(|c| c.is_whitespace() || c == '#') {
            return Err(StringValueConstructorError::InvalidValue(value.to_string()));
        }

        Ok(Self(value.to_string()))
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
        if value
            .chars()
            .any(|c| !c.is_whitespace() || c == '\n' || c == '\r')
        {
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
