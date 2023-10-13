/// the success criteria to use
#[derive(Debug)]
pub enum Criteria {
    /// a hard error that should be fixed
    Error,
    /// a warning that may be an issue
    Warning,
    /// a generic notice to help accessibility needs
    Notice,
}

impl Criteria {
    /// get rule id to string
    pub fn as_str(&self) -> &'static str {
        match self {
            Criteria::Error => "error",
            Criteria::Warning => "warning",
            Criteria::Notice => "notice",
        }
    }
}

/// wcag principle to follow
#[derive(Debug)]
pub enum Principle {
    /// Provide text alternatives for any non-text content so that it can be changed into other forms people need, such as large print, braille, speech, symbols or simpler language.
    Perceivable,
    /// Make all functionality available from a keyboard.
    Operable,
    /// Make text content readable and understandable.
    Understandable,
    /// Maximize compatibility with current and future user agents, including assistive technologies.
    Robust,
}

impl Principle {
    /// the principle to string code
    pub fn as_str(&self) -> &'static str {
        match self {
            Principle::Perceivable => "Principle1",
            Principle::Operable => "Principle2",
            Principle::Understandable => "Principle3",
            Principle::Robust => "Principle4",
        }
    }
    /// the principle index
    pub fn as_index(&self) -> &'static str {
        match self {
            Principle::Perceivable => "1",
            Principle::Operable => "2",
            Principle::Understandable => "3",
            Principle::Robust => "4",
        }
    }
}

/// wcag principle to follow
#[derive(Debug)]
pub enum Guideline {
    /// Provide text alternatives for any non-text content so that it can be changed into other forms people need.
    TextAlternatives,
    /// Create content that can be presented in different ways (for example simpler layout) without losing information or structure.
    Adaptable,
    /// Make it easier for users to see and hear content including separating foreground from background.
    Distinguishable,
    /// Provide users enough time to read and use content.
    EnoughTime,
    /// Do not design content in a way that is known to cause seizures.
    Seizures,
    /// Provide ways to help users navigate, find content, and determine where they are.
    Navigable,
    /// Make text content readable and understandable.
    Readable,
    /// Make Web pages appear and operate in predictable ways.
    Predictable,
    /// Maximize compatibility with current and future user agents, including assistive technologies.
    Compatible,
}

impl Guideline {
    /// the guideline to string code
    pub fn as_str(&self) -> &'static str {
        match self {
            Guideline::TextAlternatives => "Guideline1_1",
            Guideline::Adaptable => "Guideline1_3",
            Guideline::Distinguishable => "Guideline1_4",
            Guideline::EnoughTime => "Guideline2_2",
            Guideline::Seizures => "Guideline2_3",
            Guideline::Navigable => "Guideline2_4",
            Guideline::Readable => "Guideline3_1",
            Guideline::Predictable => "Guideline3_2",
            Guideline::Compatible => "Guideline4_1",
        }
    }
    /// the principle index
    pub fn as_index(&self) -> &'static str {
        let s = self.as_str();
        &s[9..s.len()]
    }
}
