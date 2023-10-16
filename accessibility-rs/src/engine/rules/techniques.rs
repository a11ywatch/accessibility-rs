use strum_macros::IntoStaticStr;

#[derive(PartialOrd, Ord, std::cmp::Eq, PartialEq, Hash, Debug, IntoStaticStr, Clone)]
/// techniques for WCAG <https://www.w3.org/TR/WCAG20-TECHS/>
pub enum Techniques {
    /// <https://www.w3.org/TR/WCAG20-TECHS/H25>
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
    /// <https://www.w3.org/TR/WCAG20-TECHS/F77>
    F77,
}

impl Techniques {
    /// get rule id to string
    pub fn as_str(&self) -> &'static str {
        self.into()
    }
}
