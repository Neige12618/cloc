#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeState {
    /// 在多行注释内
    InMultilineComent,
    /// 在原始字符串内
    InVerbatimQuote,
    /// 其他
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineType {
    /// 代码
    Code,
    /// 空行
    Blank,
    /// 注释
    Comment,
}
