#[macro_use]
extern crate lazy_static;

/// the main engine for audits.
mod engine;
/// locales for translations.
mod i18n;
/// app utilities.
mod utils;

pub use crate::engine::audit::auditor::Auditor;
pub use crate::engine::issue::Issue;
pub use accessibility_scraper::ElementRef;

/// configs for the audit
#[derive(Default)]
pub struct AuditConfig {
    /// the html source code
    pub html: &'static str,
    /// the css rules to apply
    pub css: &'static str,
    /// extract bounding box of elements
    pub bounding_box: bool,
    /// the locale of the audit translations
    pub locale: &'static str,
}

impl AuditConfig {
    /// a new audit configuration
    pub fn new(
        html: &'static str,
        css: &'static str,
        bounding_box: bool,
        locale: &'static str,
    ) -> Self {
        AuditConfig {
            html,
            css,
            bounding_box,
            locale,
        }
    }

    /// basic audit
    pub fn basic(html: &'static str) -> Self {
        AuditConfig {
            html,
            ..Default::default()
        }
    }
}

/// audit a web page passing the html and css rules.
pub fn audit(config: &AuditConfig) -> Vec<Issue> {
    let document = accessibility_scraper::Html::parse_document(config.html);
    let mut nth_index_cache = selectors::NthIndexCache::from(Default::default());
    let auditor = Auditor::new(
        &document,
        &config.css,
        engine::styles::css_cache::build_matching_context(&mut nth_index_cache),
        config.bounding_box,
    );
    engine::audit::wcag::WCAG3AA::audit(&auditor)
}
