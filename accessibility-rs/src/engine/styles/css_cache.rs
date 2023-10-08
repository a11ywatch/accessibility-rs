/// build matching context
pub fn build_matching_context<'a>(
    nth_index_cache: &'a mut selectors::NthIndexCache,
) -> selectors::matching::MatchingContext<'a, accessibility_scraper::selector::Simple> {
    selectors::matching::MatchingContext::new(
        selectors::matching::MatchingMode::Normal,
        None,
        Some(nth_index_cache),
        selectors::matching::QuirksMode::NoQuirks,
    )
}
