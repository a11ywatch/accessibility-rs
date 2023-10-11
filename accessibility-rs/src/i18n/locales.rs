use crate::engine::rules::rule::Rule;

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

/// get message config type
pub fn get_message_i18n(rule: &Rule, section: &str, lang: &str) -> String {
    let base = [rule.guideline.as_index(), rule.principle.as_index()].join("_") + "_";
    let message = if section.is_empty() {
        [rule.rule_id.as_str()].join(".").to_string()
    } else {
        [rule.rule_id.as_str(), section].join(".").to_string()
    };
    let message = [base.as_str(), message.as_str()].join("").to_string();

    t!(&message, lang = lang)
}
