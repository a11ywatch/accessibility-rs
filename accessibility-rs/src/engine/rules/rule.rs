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
    pub fn new(
        valid: bool,
        id: &'static str,
        elements: Vec<String>,
        message: String,
    ) -> Self {
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

/// the rule validation method that should be performed.
#[derive(Debug)]
pub struct Rule {
    /// the message id of the rule to point to the locale
    pub rule_id: Techniques,
    /// the type of rule
    pub issue_type: IssueType,
    /// validate a test returns (valid, rule, selectors)
    pub validate: fn(&Vec<(ElementRef<'_>, Option<DefaultKey>)>, &str) -> Validation,
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
        rule_id: Techniques,
        issue_type: IssueType,
        principle: Principle,
        guideline: Guideline,
        success_criteria: &'static str,
        validate: fn(&Vec<(ElementRef<'_>, Option<DefaultKey>)>, &str) -> Validation,
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
