use crate::engine::issue::Issue;
use crate::engine::rules::rule::RuleValidation;
use crate::engine::rules::wcag_rule_map::RULES_A;
use crate::Auditor;
#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[derive(Default)]
/// baseline for all rules
pub struct WCAGAAA;

/// wcag rules to test for
impl WCAGAAA {
    /// audit html against WCAGAAA standards
    #[cfg(feature = "rayon")]
    pub fn audit(auditor: (Auditor<'_>, Option<taffy::TaffyTree>)) -> Vec<Issue> {
        use crate::engine::audit::audit_utils::evaluate_rules_in_parallel;

        if auditor.0.document.tree.nodes().len() <= 5500 {
            WCAGAAA::audit_sync(auditor)
        } else {
            let (s, r) = crossbeam_channel::unbounded();

            auditor.0.tree.par_iter().for_each(|node| {
                if let Some(rules) = RULES_A.get(&*node.0) {
                    evaluate_rules_in_parallel(rules, &node, &auditor.0, &s);
                }
            });

            drop(s);

            r.iter().collect()
        }
    }

    /// audit html against WCAGAAA standards
    pub fn audit_sync(auditor: (Auditor<'_>, Option<taffy::TaffyTree>)) -> Vec<Issue> {
        use crate::engine::audit::audit_utils::push_issue_base;
        let mut issues: Vec<Issue> = Vec::new();

        for node in auditor.0.tree.iter() {
            match RULES_A.get(&*node.0) {
                Some(rules) => {
                    for rule in rules {
                        match (rule.validate)(&node.1, &auditor.0) {
                            RuleValidation::Single(validation) => push_issue_base(
                                validation,
                                rule,
                                &node.0,
                                &auditor.0.locale,
                                &mut issues,
                            ),
                            RuleValidation::Multi(validation) => {
                                for v in validation {
                                    push_issue_base(
                                        v,
                                        rule,
                                        &node.0,
                                        &auditor.0.locale,
                                        &mut issues,
                                    )
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

    /// audit html against WCAGAAA standards
    #[cfg(not(feature = "rayon"))]
    pub fn audit(auditor: (Auditor<'_>, Option<taffy::TaffyTree>)) -> Vec<Issue> {
        WCAGAAA::audit_sync(auditor)
    }
}
