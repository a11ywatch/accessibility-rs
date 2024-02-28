use slotmap::DefaultKey;

use crate::engine::rules::techniques::Techniques;
use crate::engine::rules::wcag_base::{Guideline, IssueType, Principle};
use crate::ElementRef;

/// the validation response
#[derive(Default, Debug)]
pub struct Validation {
    /// is valid
    pub valid: bool,
    /// the sub-technique
    pub id: &'static str,
    /// elements that match the issue
    pub elements: Vec<String>,
    /// the message of the error
    pub message: String,
}

impl Validation {
    /// helper to create validation
    pub fn new(valid: bool, id: &'static str, elements: Vec<String>, message: String) -> Self {
        Self {
            valid,
            id,
            elements,
            message,
        }
    }
    /// basic validation
    pub fn new_issue(valid: bool, id: &'static str) -> Self {
        Self {
            valid,
            id,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
/// techniques for a rule
pub enum Technique {
    /// a single technique
    Single(Techniques),
    /// multiple techniques
    Multi(Vec<Techniques>),
}

impl Technique {
    /// technique(s) into string
    pub fn into_str(&self) -> String {
        match self {
            Technique::Multi(tech) => tech
                .iter()
                .map(|x| x.as_str())
                .collect::<Vec<_>>()
                .join(",")
                .into(),
            Technique::Single(tech) => tech.as_str().into(),
        }
    }
}

impl From<Techniques> for Technique {
    fn from(t: Techniques) -> Self {
        Technique::Single(t)
    }
}

impl From<Vec<Techniques>> for Technique {
    fn from(t: Vec<Techniques>) -> Self {
        Technique::Multi(t)
    }
}

/// validation rule(s) that should be handled
pub enum RuleValidation {
    /// a single rule for validation
    Single(Validation),
    /// multiple validation rules applied
    Multi(Vec<Validation>),
}

impl From<Validation> for RuleValidation {
    fn from(t: Validation) -> Self {
        RuleValidation::Single(t)
    }
}

impl From<Vec<Validation>> for RuleValidation {
    fn from(t: Vec<Validation>) -> Self {
        RuleValidation::Multi(t)
    }
}

type ValidateFn = fn(&Vec<(ElementRef<'_>, Option<DefaultKey>)>, &crate::Auditor<'_>) -> RuleValidation;

/// the rule validation method that should be performed.
#[derive(Debug)]
pub struct Rule {
    /// the message id of the rule to point to the locale
    pub rule_id: Technique,
    /// the type of rule
    pub issue_type: IssueType,
    /// validate a test returns (valid, rule, selectors)
    pub validate: ValidateFn,
    /// the principle type
    pub principle: Principle,
    /// the guideline to follow
    pub guideline: Guideline,
    /// the success criteria
    pub success_criteria: &'static str,
}

impl Rule {
    /// a new rule type
    pub fn new(
        rule_id: Technique,
        issue_type: IssueType,
        principle: Principle,
        guideline: Guideline,
        success_criteria: &'static str,
        validate: ValidateFn,
    ) -> Rule {
        Rule {
            rule_id,
            issue_type,
            guideline,
            principle,
            success_criteria,
            validate,
        }
    }
}
