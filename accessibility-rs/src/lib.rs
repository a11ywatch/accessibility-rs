#![warn(missing_docs)]

//! Audit html to see how it complies with WCAG
//! standards.
//!
//! Accessibility-RS is a powerful web accessibility
//! engine that can replicate websites without
//! a browser to get complex accessibility reports.
//!
//! # How to use Accessibility-RS
//!
//! There are a couple of ways to use Accessibility-RS:
//!
//! - **Audit** perform an audit against an html page.
//!   - [`audit`] is used to audit a web page for issues.
//!
//! [`audit`]: fn.audit.html#method.audit
//!
//! # Examples
//!
//! A basic WCAG audit for a website:
//!
//! ```no_run
//! use accessibility_rs::{audit, AuditConfig};
//!
//! fn main() {
//!     // pass in the html, and css if the page came from a headless browser
//!     let config = AuditConfig::basic("<html>...</html>");
//!     let audit = audit(config);
//!     println!("{:?}", audit);
//! }
//! ```
//!

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rust_i18n;

/// the main engine for audits.
mod engine;
/// locales for translations.
mod i18n;
/// app utilities.
mod utils;

pub use crate::engine::audit::auditor::Auditor;
pub use crate::engine::issue::Issue;
pub use accessibility_scraper::ElementRef;

i18n!("locales");

/// configs for the audit
#[derive(Default)]
pub struct AuditConfig<'a> {
    /// the html source code
    pub html: &'a str,
    /// the css rules to apply
    pub css: &'a str,
    /// extract bounding box of elements
    pub bounding_box: bool,
    /// the locale of the audit translations
    pub locale: &'a str,
}

impl<'a> AuditConfig<'a> {
    /// a new audit configuration
    pub fn new(
        html: &'a str,
        css: &'a str,
        bounding_box: bool,
        locale: &'a str,
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
pub fn audit(config: AuditConfig) -> Vec<Issue> {
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
