use std::vec;

#[derive(PartialOrd, Ord, std::cmp::Eq, PartialEq, Hash, Debug)]
/// techniques for WCAG <https://www.w3.org/TR/WCAG20-TECHS/>
pub enum Techniques {
    /// <https://www.w3.org/TR/WCAG20-TECHS/H25.html>
    H25,
    /// <https://www.w3.org/TR/WCAG20-TECHS/H32.html>
    H32,
}

impl Techniques {
    /// get rule id to string
    pub fn as_str(&self) -> &'static str {
        match self {
            Techniques::H25 => "H25",
            Techniques::H32 => "H32",
        }
    }
    /// get pairs for a rule
    pub fn pairs(&self) -> Vec<&'static str> {
        match self {
            Techniques::H25 => vec!["H25.1.NoTitleEl", "H25.1.EmptyTitle"],
            Techniques::H32 => vec!["H32.2"],
        }
    }
}
