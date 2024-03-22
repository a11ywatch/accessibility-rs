use crate::engine::issue::Issue;
use crate::engine::rules::rule::{Rule, RuleValidation, Validation};
use crate::engine::rules::wcag_rule_map::RULES_A;
use crate::i18n::locales::get_message_i18n;
use crate::Auditor;

/// validate rule and push issue
#[inline]
fn push_issue(
    validation: Validation,
    rule: &Rule,
    context: &str,
    lang: &str,
    issues: &mut Vec<Issue>,
) {
    if !validation.valid {
        issues.push(Issue::new(
            if !validation.message.is_empty() {
                validation.message.into()
            } else {
                get_message_i18n(&rule, &validation.id, &lang)
            },
            &context,
            &[
                "WCAGAAA",
                rule.principle.as_str(),
                rule.guideline.as_str(),
                &rule.rule_id.into_str(),
            ]
            .join("."),
            rule.issue_type.as_str(),
            validation.elements,
        ));
    }
}

/// baseline for all rules
#[derive(Default)]
pub struct WCAGAAA;

/// wcag rules to test for
impl WCAGAAA {
    /// audit html against WCAGAAA standards
    pub fn audit(auditor: (Auditor<'_>, Option<taffy::TaffyTree>)) -> Vec<Issue> {
        let mut issues: Vec<Issue> = Vec::new();

        for node in auditor.0.tree.iter() {
            match RULES_A.get(&*node.0) {
                Some(rules) => {
                    for rule in rules {
                        match (rule.validate)(&node.1, &auditor.0) {
                            RuleValidation::Single(validation) => push_issue(
                                validation,
                                rule,
                                &node.0,
                                &auditor.0.locale,
                                &mut issues,
                            ),
                            RuleValidation::Multi(validation) => {
                                for v in validation {
                                    push_issue(v, rule, &node.0, &auditor.0.locale, &mut issues)
                                }
                            }
                        };
                    }
                }
                _ => (),
            }
        }

        issues
    }
}
