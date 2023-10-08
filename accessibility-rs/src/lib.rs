#[macro_use]
extern crate lazy_static;

/// the main engine for audits.
mod engine;
/// locales for translations.
mod i18n;
/// app utilities.
mod utils;

use crate::engine::audit::auditor::Auditor;
use crate::engine::issue::Issue;
use accessibility_scraper::ElementRef;

/// audit a web page passing the html and css rules.
pub fn audit(html: &str, css_rules: &str) -> Vec<Issue> {
    let document = accessibility_scraper::Html::parse_document(html);
    let mut nth_index_cache = selectors::NthIndexCache::from(Default::default());
    let auditor = Auditor::new(
        &document,
        &css_rules,
        engine::styles::css_cache::build_matching_context(&mut nth_index_cache),
    );
    engine::audit::wcag::WCAG3AA::audit(&auditor)
}
