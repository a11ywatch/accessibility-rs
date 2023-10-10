use crate::engine::rules::ids::Techniques;
use std::collections::BTreeMap;

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

/// the context of the issue
impl Messages {
    /// create a new message
    pub fn new(en: M, es: M, de: M, ja: M) -> Messages 
    {
        Messages {
            en,
            es,
            de,
            ja,
            pt_pt: &"",
            pt_br: &"",
            zh_cn: &"",
            zh_tw: &"",
            hi: &"",
        }
    }
}

/// parse
pub fn get_message(rule_id: &Techniques, section: &str, lang: &str) -> &'static str {
    let rule_id = rule_id.as_str();
    let message = if section.is_empty() {
        LOCALES.get(&rule_id)
    } else {
        LOCALES.get([rule_id, section].join(".").as_str())
    };

    match message {
        Some(m) => match lang {
            "en" => m.en,
            "es" => m.es,
            _ => Default::default(),
        },
        _ => Default::default(),
    }
}

lazy_static! {
    /// message for an issue
    pub static ref LOCALES: BTreeMap<&'static str, Messages> = {
        BTreeMap::from([
            (Techniques::H25.pairs()[0], Messages::new(
                "A title should be provided for the document, using a non-empty title element in the head section.", 
                "Se debe proporcionar un título para el documento, utilizando un elemento de título no vacío en la sección head.", 
                "",
                "head セクションの空でない title 要素を使って、文書にタイトルをつけるべきです。"
            )),
            (Techniques::H25.pairs()[1], Messages::new(&"The title element in the head section should be non-empty.", "", "", "")),
            (Techniques::H32.pairs()[0], Messages::new(&r###"Form does not contain a submit button (input type="submit", input type="image", or button type="submit")."###, "", "", "")),
            (Techniques::H64.pairs()[0], Messages::new(&"Iframe element requires a non-empty title attribute that identifies the frame.", "", "", "")),
            (Techniques::H57.pairs()[0], Messages::new(&"The html element should have a lang or xml:lang attribute which describes the language of the document.", "", "", "")),
            (Techniques::H57.pairs()[1], Messages::new(&"The language specified in the lang attribute of the document element does not appear to be well-formed.", "", "", "")),
            (Techniques::H57.pairs()[2], Messages::new(&"The language specified in the xml:lang attribute of the document element does not appear to be well-formed.", "", "", "")),
            (Techniques::F40.pairs()[0], Messages::new(&"Meta refresh tag used to redirect to another page, with a time limit that is not zero. Users cannot control this time limit.", "", "", "")),
            (Techniques::F41.pairs()[0], Messages::new(&"Meta refresh tag used to refresh the current page. Users cannot control the time limit for this refresh.", "", "", "")),
            (Techniques::F47.pairs()[0], Messages::new(&"Blink elements cannot satisfy the requirement that blinking information can be stopped within five seconds.", "", "", "")),
        ])
    };
}
