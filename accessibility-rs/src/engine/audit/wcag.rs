use crate::engine::issue::Issue;
use crate::engine::rules::rule::{Rule, RuleValidation, Validation};
use crate::engine::rules::wcag_rule_map::RULES_A;
use crate::i18n::locales::get_message_i18n;
use crate::Auditor;

/// baseline for all rules
#[derive(Default)]
pub struct WCAGAAA;

/// validate rule and push issue
fn push_issue(
    validation: Validation,
    rule: &Rule,
    context: &str,
    lang: &str,
    issues: &mut Vec<Issue>,
) {
    let message = validation.message;

    if !validation.valid {
        let message = if !message.is_empty() {
            message.into()
        } else {
            get_message_i18n(&rule, &validation.id, &lang)
        };
        issues.push(Issue::new(
            message,
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

/// wcag rules to test for
impl WCAGAAA {
    /// audit html against WCAGAAA standards
    pub fn audit(auditor: &Auditor<'_>) -> Vec<Issue> {
        // TODO: push rules found to MAP that are different across nodes to combine the selectors Ex: <a> and <link> both have the href attribute.
        let mut issues: Vec<Issue> = Vec::new();

        for node in &auditor.tree {
            if RULES_A.contains_key(&*node.0) {
                let rules = RULES_A.get(&*node.0);
                match rules {
                    Some(rules) => {
                        for rule in rules {
                            let validation = (rule.validate)(&node.1, &auditor.locale);
                            match validation {
                                RuleValidation::Single(validation) => push_issue(
                                    validation,
                                    rule,
                                    &node.0,
                                    &auditor.locale,
                                    &mut issues,
                                ),
                                RuleValidation::Multi(validation) => {
                                    for v in validation {
                                        push_issue(v, rule, &node.0, &auditor.locale, &mut issues)
                                    }
                                }
                            };
                        }
                    }
                    _ => (),
                }
            }
        }

        issues
    }
}
