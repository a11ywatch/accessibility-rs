use std::vec;
use strum_macros::IntoStaticStr;

#[derive(PartialOrd, Ord, std::cmp::Eq, PartialEq, Hash, Debug, IntoStaticStr)]
/// techniques for WCAG <https://www.w3.org/TR/WCAG20-TECHS/>
pub enum Techniques {
    /// <https://www.w3.org/TR/WCAG20-TECHS/H25.html>
    H25,
    /// <https://www.w3.org/TR/WCAG20-TECHS/H32.html>
    H32,
    /// <https://www.w3.org/TR/WCAG20-TECHS/H57>
    H57,
    /// <https://www.w3.org/TR/WCAG20-TECHS/F40>
    F40,
    /// <https://www.w3.org/TR/WCAG20-TECHS/F41>
    F41,
    /// <https://www.w3.org/TR/WCAG20-TECHS/F47>
    F47,
}

impl Techniques {
    /// get rule id to string
    pub fn as_str(&self) -> &'static str {
        self.into()
    }
    /// get pairs for a rule
    pub fn pairs(&self) -> Vec<&'static str> {
        match self {
            Techniques::H25 => vec!["H25.1.NoTitleEl", "H25.1.EmptyTitle"],
            Techniques::H32 => vec!["H32.2"],
            Techniques::H57 => vec!["H57.2", "H57.3.Lang", "H57.3.XmlLang"],
            Techniques::F40 => vec!["F40.2"],
            Techniques::F41 => vec!["F41.2"],
            Techniques::F47 => vec!["F47"],
        }
    }
}
