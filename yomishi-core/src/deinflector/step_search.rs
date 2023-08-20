/// Iterates the string by taking one character from the end each time
///
/// # Example
/// ```
/// let mut iter = StepSearch::new_from_str("しょく");
///
/// assert_eq!(iter.next(), Some("しょく"));
/// assert_eq!(iter.next(), Some("しょ"));
/// assert_eq!(iter.next(), Some("し"));
/// assert_eq!(iter.next(), None);
/// ```
pub struct StepSearch<'a> {
    source: &'a str,
    i: usize,
}

impl<'a> StepSearch<'a> {
    pub fn new_from_str(source: &'a str) -> Self {
        Self { source, i: 0 }
    }
}

impl<'a> Iterator for StepSearch<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        if self.i > self.source.char_indices().count() {
            None
        } else {
            Some(
                &self.source[..self
                    .source
                    .char_indices()
                    .nth(self.source.char_indices().count() - (self.i - 1))
                    .map(|(a, _)| a)
                    .unwrap_or(self.source.len())],
            )
        }
    }
}
