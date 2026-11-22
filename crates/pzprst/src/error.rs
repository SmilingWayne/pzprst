use std::fmt;

/// Something pzprjs would silently accept, repair or drop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticCode {
    TrailingData,
    TruncatedBody,
    UnexpectedChar,
    UnknownFlag,
    UnknownParameter,
    IgnoredState,
    IgnoredTrace,
    LossyUrl,
    ValueOutOfRange,
    DroppedMetadata,
    DroppedHistory,
    DroppedTrial,
}

impl DiagnosticCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TrailingData => "TrailingData",
            Self::TruncatedBody => "TruncatedBody",
            Self::UnexpectedChar => "UnexpectedChar",
            Self::UnknownFlag => "UnknownFlag",
            Self::UnknownParameter => "UnknownParameter",
            Self::IgnoredState => "IgnoredState",
            Self::IgnoredTrace => "IgnoredTrace",
            Self::LossyUrl => "LossyUrl",
            Self::ValueOutOfRange => "ValueOutOfRange",
            Self::DroppedMetadata => "DroppedMetadata",
            Self::DroppedHistory => "DroppedHistory",
            Self::DroppedTrial => "DroppedTrial",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub message: String,
}

impl Diagnostic {
    pub fn new(code: DiagnosticCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code.as_str(), self.message)
    }
}

/// Collected diagnostics; repeated codes are kept once with a counter.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Diagnostics(Vec<Diagnostic>);

impl Diagnostics {
    pub fn push(&mut self, code: DiagnosticCode, message: impl Into<String>) {
        if self.0.iter().any(|d| d.code == code) {
            return;
        }
        self.0.push(Diagnostic::new(code, message));
    }

    pub fn as_slice(&self) -> &[Diagnostic] {
        &self.0
    }

    pub(crate) fn check_strict(self, strict: bool) -> Result<Self> {
        if strict && !self.0.is_empty() {
            Err(Error::Strict(self.0))
        } else {
            Ok(self)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidUrl(String),
    UnknownPuzzle(String),
    UnsupportedPuzzle(String),
    UnsupportedUrlType(String),
    InvalidSize(String),
    InvalidFileIo { line: usize, message: String },
    UnsupportedFileVersion(String),
    ValueOutOfRange(String),
    StateDecode(String),
    StateMismatch(String),
    TraceNotSupported,
    ResourceLimit(String),
    UnknownRule(String),
    InvalidTransform(String),
    Strict(Vec<Diagnostic>),
}

impl Error {
    /// Stable variant name, used by bindings to pick an exception class.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::InvalidUrl(_) => "InvalidUrl",
            Self::UnknownPuzzle(_) => "UnknownPuzzle",
            Self::UnsupportedPuzzle(_) => "UnsupportedPuzzle",
            Self::UnsupportedUrlType(_) => "UnsupportedUrlType",
            Self::InvalidSize(_) => "InvalidSize",
            Self::InvalidFileIo { .. } => "InvalidFileIo",
            Self::UnsupportedFileVersion(_) => "UnsupportedFileVersion",
            Self::ValueOutOfRange(_) => "ValueOutOfRange",
            Self::StateDecode(_) => "StateDecode",
            Self::StateMismatch(_) => "StateMismatch",
            Self::TraceNotSupported => "TraceNotSupported",
            Self::ResourceLimit(_) => "ResourceLimit",
            Self::UnknownRule(_) => "UnknownRule",
            Self::InvalidTransform(_) => "InvalidTransform",
            Self::Strict(_) => "Strict",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUrl(m) => write!(f, "invalid URL: {m}"),
            Self::UnknownPuzzle(p) => write!(f, "unknown puzzle type: {p:?}"),
            Self::UnsupportedPuzzle(p) => write!(f, "puzzle type not supported by pzprst yet: {p}"),
            Self::UnsupportedUrlType(m) => write!(f, "unsupported URL type: {m}"),
            Self::InvalidSize(m) => write!(f, "invalid board size: {m}"),
            Self::InvalidFileIo { line, message } => write!(f, "invalid FileIO at line {line}: {message}"),
            Self::UnsupportedFileVersion(m) => write!(f, "unsupported FileIO version: {m}"),
            Self::ValueOutOfRange(m) => write!(f, "value out of range: {m}"),
            Self::StateDecode(m) => write!(f, "cannot decode state: {m}"),
            Self::StateMismatch(m) => write!(f, "state does not match the puzzle: {m}"),
            Self::TraceNotSupported => write!(f, "trace (operation history) URLs are not supported; pass ignore_trace to read only the state"),
            Self::ResourceLimit(m) => write!(f, "resource limit exceeded: {m}"),
            Self::UnknownRule(m) => write!(f, "unknown rule: {m}"),
            Self::InvalidTransform(m) => write!(f, "invalid transform: {m}"),
            Self::Strict(ds) => {
                write!(f, "strict mode rejected input:")?;
                for d in ds {
                    write!(f, " [{d}]")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
