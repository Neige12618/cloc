#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeState {
    InMultilineComent,
    InVerbatimQuote,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineType {
    Code,
    Blank,
    Comment,
}
