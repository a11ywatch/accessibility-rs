use crate::engine::issue::Issue;
use crate::engine::rules::wcag_rule_map::RULES_A;
use crate::i18n::locales::get_message_i18n;
use crate::Auditor;

/// baseline for all rules
#[derive(Default)]
pub struct WCAG3AA;

/// wcag rules to test for
impl WCAG3AA {
    /// init the rules
    pub fn audit(auditor: &Auditor<'_>) -> Vec<Issue> {
        // TODO: push rules found to MAP that are different across nodes to combine the selectors
        let mut issues: Vec<Issue> = Vec::new();

        for node in &auditor.tree {
            if RULES_A.contains_key(&*node.0) {
                let rules = RULES_A.get(&*node.0);
                match rules {
                    Some(rules) => {
                        for rule in rules {
                            let validation = (rule.validate)(&node.1, &auditor.locale);
                            let message = validation.message;

                            if !validation.valid {
                                let message = if !message.is_empty() {
                                    message.into()
                                } else {
                                    get_message_i18n(&rule, &validation.id, &auditor.locale)
                                };
                                issues.push(Issue::new(
                                    message,
                                    &node.0,
                                    &[
                                        "WCAGAAA",
                                        rule.principle.as_str(),
                                        rule.guideline.as_str(),
                                        rule.rule_id.as_str(),
                                    ]
                                    .join("."),
                                    rule.issue_type.as_str(),
                                    validation.elements,
                                ));
                            }
                        }
                    }
                    _ => (),
                }
            }
        }

        issues
    }
}
