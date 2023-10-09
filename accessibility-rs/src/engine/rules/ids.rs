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
    H57
}

impl Techniques {
    /// get rule id to string
    pub fn as_str(&self) -> &'static str {
        // todo: make macro
       self.into()
    }
    /// get pairs for a rule
    pub fn pairs(&self) -> Vec<&'static str> {
        match self {
            Techniques::H25 => vec!["H25.1.NoTitleEl", "H25.1.EmptyTitle"],
            Techniques::H32 => vec!["H32.2"],
            Techniques::H57 => vec!["H57.2", "H57.3.Lang", "H57.3.XmlLang"],
        }
    }
}
