use crate::engine::styles::errors::RuleParseErrorKind;
use accessibility_scraper::selector::Simple;
use cssparser::{AtRuleParser, ParseError, QualifiedRuleParser};
use std::sync::Arc;

#[derive(Debug)]
/// Css rules to match selectors
pub enum CssRule {
    /// the style rules for css
    StyleRule {
        /// css selectors list with selector trait
        selectors: selectors::SelectorList<Simple>,
        /// a css block parsed
        block: Arc<String>,
    },
}

/// css parser
pub struct Parser;

impl<'i> selectors::parser::Parser<'i> for Parser {
    type Impl = Simple;
    type Error = RuleParseErrorKind<'i>;
}

/// css selector list
pub type SelectorList = selectors::SelectorList<Simple>;
// pub type Selector = selectors::parser::Selector<Simple>;

/// css rules parser
pub struct RulesParser;

impl<'i> QualifiedRuleParser<'i> for RulesParser {
    type Prelude = SelectorList;
    type QualifiedRule = CssRule;
    type Error = RuleParseErrorKind<'i>;

    fn parse_prelude<'t>(
        &mut self,
        parser: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        SelectorList::parse(&Parser, parser)
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        _location: cssparser::SourceLocation,
        _parser: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        Ok(CssRule::StyleRule {
            selectors: prelude,
            block: Arc::new(String::new()),
        })
    }
}

impl<'i> AtRuleParser<'i> for RulesParser {
    type PreludeBlock = ();
    type PreludeNoBlock = ();
    type AtRule = CssRule;
    type Error = RuleParseErrorKind<'i>;
}
