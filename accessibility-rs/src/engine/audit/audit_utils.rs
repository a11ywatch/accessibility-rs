use crate::engine::issue::Issue;
use crate::engine::rules::rule::{Rule, Validation};
use crate::i18n::locales::get_message_i18n;

/// validate rule and push issue base
pub fn push_issue_base(
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

/// validate rule and push issue
#[cfg(feature = "rayon")]
pub fn push_issue(
    validation: Validation,
    rule: &Rule,
    context: &str,
    lang: &str,
    s: &crossbeam_channel::Sender<Issue>,
) {
    // logic remains the same, notice the lock acquisition and pushing to the vector.
    if !validation.valid {
        let issue = Issue::new(
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
        );

        match s.send(issue) {
            _ => (),
        }
    }
}

#[cfg(not(feature = "rayon"))]
/// validate rule and push issue
pub fn push_issue(
    validation: Validation,
    rule: &Rule,
    context: &str,
    lang: &str,
    issues: &mut Vec<Issue>,
) {
    push_issue_base(validation, rule, context, lang, issues)
}

#[cfg(feature = "rayon")]
/// validate rule and push issue parallel
pub fn evaluate_rules_in_parallel(
    rules: &[crate::engine::rules::rule::Rule],
    node: &(
        &&str,
        &Vec<(crate::ElementRef<'_>, std::option::Option<taffy::NodeId>)>,
    ),
    auditor: &crate::Auditor,
    s: &crossbeam_channel::Sender<Issue>,
) {
    use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
    rules.par_iter().for_each(|rule| {
        match (rule.validate)(&node.1, &auditor) {
            crate::engine::rules::rule::RuleValidation::Single(validation) => {
                push_issue(validation, rule, &node.0, &auditor.locale, s)
            }
            crate::engine::rules::rule::RuleValidation::Multi(validations) => {
                validations.into_par_iter().for_each(|validation| {
                    push_issue(validation, rule, &node.0, &auditor.locale, s)
                });
            }
        };
    });
}
