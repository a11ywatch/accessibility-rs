use crate::engine::issue::Issue;
use crate::engine::rules::rule::RuleValidation;
use crate::engine::rules::wcag_rule_map::RULES_A;
use crate::Auditor;
#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[derive(Default)]
/// Baseline for all rules
pub struct WCAGAAA;

/// WCAG rules to test for
impl WCAGAAA {
    /// Audit html against WCAGAAA standards
    #[cfg(not(feature = "tokio"))]
    pub fn run_audit(auditor: (Auditor<'_>, Option<taffy::TaffyTree>)) -> Vec<Issue> {
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

    /// Audit html against WCAGAAA standards
    #[cfg(feature = "tokio")]
    pub async fn run_audit(auditor: (Auditor<'_>, Option<taffy::TaffyTree>)) -> Vec<Issue> {
        use crate::engine::audit::audit_utils::push_issue_base;
        use tokio_stream::{self as stream, StreamExt};
        let mut issues: Vec<Issue> = Vec::new();
        let stream = stream::iter(auditor.0.tree.iter());
        tokio::pin!(stream);

        while let Some(node) = stream.next().await {
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

    /// Audit html against WCAGAAA standards
    #[cfg(all(feature = "rayon", not(feature = "spider"), not(feature = "tokio")))]
    pub fn audit(auditor: (Auditor<'_>, Option<taffy::TaffyTree>)) -> Vec<Issue> {
        use crate::engine::audit::audit_utils::evaluate_rules_in_parallel;

        if auditor.0.document.tree.nodes().len() <= 5500 {
            WCAGAAA::run_audit(auditor)
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

    /// Audit html against WCAGAAA standards
    #[cfg(all(
        not(feature = "rayon"),
        not(feature = "spider"),
        not(feature = "tokio")
    ))]
    pub fn audit(auditor: (Auditor<'_>, Option<taffy::TaffyTree>)) -> Vec<Issue> {
        WCAGAAA::run_audit(auditor)
    }

    /// Audit html against WCAGAAA standards
    #[cfg(feature = "tokio")]
    pub async fn audit(auditor: (Auditor<'_>, Option<taffy::TaffyTree>)) -> Vec<Issue> {
        WCAGAAA::run_audit(auditor).await
    }
}
