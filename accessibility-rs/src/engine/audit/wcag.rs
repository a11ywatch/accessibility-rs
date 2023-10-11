use crate::engine::issue::Issue;
use crate::engine::rules::wcag_rule_map::RULES_A;
use crate::i18n::locales::{get_message_i18n, Langs};
use crate::Auditor;

/// baseline for all rules
#[derive(Default)]
pub struct WCAG3AA;

/// wcag rules to test for
impl WCAG3AA {
    /// init the rules
    pub fn audit(
        // allow tree mutation until threads or setup the tree with initial elements.
        auditor: &Auditor<'_>,
    ) -> Vec<Issue> {
        let mut issues: Vec<Issue> = Vec::new();

        // go through nodes and map to validation rules
        for node in &auditor.tree {
            if RULES_A.contains_key(&*node.0) {
                let rules = RULES_A.get(&*node.0);
                match rules {
                    Some(rules) => {
                        for rule in rules {
                            let validation = (rule.validate)(&node.0, &node.1);
                            let valid = validation.valid;
                            let section = validation.id;
                            let selector = validation.elements;
                            let message = validation.message;

                            if !valid {
                                // get locales prior or from document
                                let message = if !message.is_empty() {
                                    message.into()
                                } else {
                                    get_message_i18n(&rule, &section, &Langs::En.as_str())
                                };

                                let issue = Issue::new(
                                    message,
                                    &node.0,
                                    &[
                                        "WCAGAAA",
                                        rule.principle.as_str(),
                                        rule.guideline.as_str(),
                                        rule.rule_id.as_str(),
                                    ]
                                    .join("."),
                                    rule.criteria.as_str(),
                                    selector,
                                );
                                issues.push(issue);
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
