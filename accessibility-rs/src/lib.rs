#![warn(missing_docs)]

//! Audit html to see how it complies with WCAG
//! standards.
//!
//! accessibility-rs is a web accessibility
//! engine that can replicate websites without
//! a browser to get complex accessibility reports.
//!
//! # How to use accessibility-rs
//!
//! There are a couple of ways to use accessibility-rs:
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
//! #[cfg(not(feature = "tokio"))]
//! fn main() {
//!     let config = AuditConfig::basic(r###"<html><body><h1>My Title</h1><input type="text" placeholder="Type me"></input><img src="tabby_cat.png"></img></body></html>"###);
//!     let audit = audit(config);
//!     println!("{:?}", audit);
//! }
//!
//! #[cfg(feature = "tokio")]
//! #[tokio::main]
//! async fn main() {
//!     let config = AuditConfig::basic(r###"<html><body><h1>My Title</h1><input type="text" placeholder="Type me"></input><img src="tabby_cat.png"></img></body></html>"###);
//!     let audit = audit(config).await;
//!     println!("{:?}", audit);
//! }
//! ```
//!

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rust_i18n;

/// the main engine for accessibility auditing.
pub mod engine;
/// locales for translations.
pub mod i18n;

use crate::engine::audit::auditor::Auditor;
use crate::engine::issue::Issue;
use accessibility_scraper::ElementRef;

i18n!("locales", fallback = "en");

/// support guidelines for auditing
#[derive(Default)]
pub enum Conformance {
    /// Level AAA includes all Level A, AA, and AAA requirements
    #[default]
    WCAGAAA,
}

/// configs for the audit
#[derive(Default)]
#[cfg(feature = "tokio")]
pub struct AuditConfig {
    /// the html source code
    pub html: String,
    /// the css rules to apply
    pub css: String,
    /// extract bounding box of elements
    pub bounding_box: bool,
    /// the locale of the audit translations
    pub locale: String,
    /// the guideline spec
    pub conformance: Conformance,
}

/// configs for the audit
#[derive(Default)]
#[cfg(not(feature = "tokio"))]
pub struct AuditConfig<'a> {
    /// the html source code
    pub html: &'a str,
    /// the css rules to apply
    pub css: &'a str,
    /// extract bounding box of elements
    pub bounding_box: bool,
    /// the locale of the audit translations
    pub locale: &'a str,
    /// the guideline spec
    pub conformance: Conformance,
}

#[cfg(not(feature = "tokio"))]
impl<'a> AuditConfig<'a> {
    /// a new audit configuration
    pub fn new(html: &'a str, css: &'a str, bounding_box: bool, locale: &'a str) -> Self {
        AuditConfig {
            html: html.into(),
            css: css.into(),
            bounding_box,
            locale: locale.into(),
            ..Default::default()
        }
    }

    /// basic audit
    pub fn basic(html: &'a str) -> Self {
        AuditConfig {
            html: html.into(),
            ..Default::default()
        }
    }
}

#[cfg(feature = "tokio")]
impl AuditConfig {
    /// a new audit configuration
    pub fn new(html: &str, css: &str, bounding_box: bool, locale: &str) -> Self {
        AuditConfig {
            html: html.into(),
            css: css.into(),
            bounding_box,
            locale: locale.into(),
            ..Default::default()
        }
    }

    /// basic audit
    pub fn basic(html: &str) -> Self {
        AuditConfig {
            html: html.into(),
            ..Default::default()
        }
    }
}

/// audit a web page passing the html and css rules.
#[cfg(feature = "tokio")]
pub async fn audit(config: AuditConfig) -> Vec<Issue> {
    let document = accessibility_scraper::Html::parse_document(&config.html).await;
    let auditor = Auditor::new(&document, &config.css, config.bounding_box, &config.locale);
    engine::audit::wcag::WCAGAAA::audit(auditor).await
}

/// audit a web page passing the html and css rules.
#[cfg(not(feature = "tokio"))]
pub fn audit(config: AuditConfig) -> Vec<Issue> {
    let document = accessibility_scraper::Html::parse_document(&config.html);
    let auditor = Auditor::new(&document, &config.css, config.bounding_box, &config.locale);
    engine::audit::wcag::WCAGAAA::audit(auditor)
}
