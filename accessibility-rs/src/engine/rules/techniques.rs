use std::vec;
use strum_macros::IntoStaticStr;

#[derive(PartialOrd, Ord, std::cmp::Eq, PartialEq, Hash, Debug, IntoStaticStr)]
/// techniques for WCAG <https://www.w3.org/TR/WCAG20-TECHS/>
pub enum Techniques {
    /// <https://www.w3.org/TR/WCAG20-TECHS/H25.html>
    H25,
    /// <https://www.w3.org/TR/WCAG20-TECHS/H32.html>
    H32,
    /// <https://www.w3.org/TR/WCAG20-TECHS/H30>
    H30,
    /// <https://www.w3.org/TR/WCAG20-TECHS/H36>
    H36,
    /// <https://www.w3.org/TR/WCAG20-TECHS/H37>
    H37,
    /// <https://www.w3.org/TR/WCAG20-TECHS/H42>
    H42,
    /// <https://www.w3.org/TR/WCAG20-TECHS/H57>
    H57,
    /// <https://www.w3.org/TR/WCAG20-TECHS/H64>
    H64,
    /// <https://www.w3.org/TR/WCAG20-TECHS/H91>
    H91,
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
            Techniques::H30 => vec!["H30.2"],
            Techniques::H32 => vec!["H32.2"],
            Techniques::H36 => vec!["H36"],
            Techniques::H37 => vec!["H37"],
            Techniques::H42 => vec!["H42.2"],
            Techniques::H57 => vec!["H57.2", "H57.3.Lang", "H57.3.XmlLang"],
            Techniques::H64 => vec!["H64.1", "H64.2"],
            Techniques::H91 => vec!["H91.A.NoContent", "H91.[NodeName].Name", "H91.[NodeName].Value"],
            Techniques::F40 => vec!["F40.2"],
            Techniques::F41 => vec!["F41.2"],
            Techniques::F47 => vec!["F47"],
        }
    }
}