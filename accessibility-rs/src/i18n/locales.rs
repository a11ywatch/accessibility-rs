use crate::engine::rules::{rule::Rule, wcag_base::Guideline};

type M = &'static str;

/// messages to display for issues
#[derive(std::cmp::Eq, PartialEq, PartialOrd, Ord)]
pub struct Messages {
    /// english
    en: M,
    /// spanish
    es: M,
    /// german
    de: M,
    /// japanese
    ja: M,
    /// portugese portugal
    pt_pt: M,
    /// portugese brazil
    pt_br: M,
    /// chinese cantanese
    zh_cn: M,
    /// chinese traditional
    zh_tw: M,
    /// hindi
    hi: M,
}

pub enum Langs {
    /// english
    En,
    /// spanish
    Es,
    /// german
    De,
    /// japanese
    Ja,
    /// portugese portugal
    PtPt,
    /// portugese brazil
    PtBr,
    /// chinese cantanese
    ZhCn,
    /// chinese traditional
    ZhTw,
    /// hindi
    HI,
}

impl Langs {
    /// get the lang as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Langs::En => "en",
            Langs::Es => "es",
            Langs::De => "de",
            Langs::Ja => "ja",
            Langs::PtPt => "pt_pt",
            Langs::PtBr => "pt_br",
            Langs::ZhCn => "zh_cn",
            Langs::ZhTw => "zh_tw",
            Langs::HI => "hi",
        }
    }
}

/// get message config type raw
pub fn get_message_i18n_str_raw(guideline: &Guideline, rule_id: &str, success_criteria: &str, section: &str) -> String {
    let base = [guideline.as_index(), success_criteria].join("_") + "_";
    let message = if section.is_empty() {
        [rule_id].join(".").to_string()
    } else {
        [rule_id, section].join(".").to_string()
    };
    [base.as_str(), message.as_str()].join("").to_string()
}

/// get message config type
pub fn get_message_i18n_str(rule: &Rule, section: &str) -> String {
    get_message_i18n_str_raw(&rule.guideline, rule.rule_id.as_str(), rule.success_criteria, section)
}

/// get message
pub fn get_message_i18n(rule: &Rule, section: &str, lang: &str) -> String {
    let message = get_message_i18n_str(rule, section);

    t!(&message, locale = lang)
}
