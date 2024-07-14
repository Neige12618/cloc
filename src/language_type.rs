use crate::state::{CodeState, LineType};

include!(concat!(env!("OUT_DIR"), "/language_type.rs"));

impl LanguageType {
    /// 解析一行代码，返回状态和类型。
    pub fn parse_line(&self, line: &str, prev: CodeState) -> (CodeState, LineType) {
        let line = line.trim();
        if line.is_empty() {
            return (prev, LineType::Blank);
        }

        match prev {
            CodeState::InMultilineComent => {
                for mlc in self.multi_line_comments() {
                    // 包含结束符号，则更改状态为other
                    if line.contains(mlc.1) {
                        if line.ends_with(mlc.1) {
                            return (CodeState::Other, LineType::Comment);
                        } else {
                            return (CodeState::Other, LineType::Code);
                        }
                    }
                }
                // 不包含结束符号，状态不改变。下同
                (CodeState::InMultilineComent, LineType::Comment)
            }
            CodeState::InVerbatimQuote => {
                for vq in self.verbatim_quotes() {
                    if line.contains(vq.1) {
                        return (CodeState::Other, LineType::Code);
                    }
                }
                (CodeState::InVerbatimQuote, LineType::Code)
            }
            CodeState::Other => {
                // 获得当前language type的quote正则表达式。
                let regex = self.quotes_regex();
                let mut line = line.to_string();
                // 遍历并且替换所有匹配的quote。避免影响匹配。
                for re in regex {
                    line = re.replace_all(&line, "").to_string();
                }
                // 判断是否是行注释
                if self.line_comment().iter().any(|s| line.starts_with(s)) {
                    return (CodeState::Other, LineType::Comment);
                }
                // 判断是否是多行注释
                for mlc in self.multi_line_comments() {
                    if let Some(p) = line.find(mlc.0) {
                        if let Some(q) = line.find(mlc.1) {
                            // 包含多行注释结束，则判断是否存在code。
                            // 如果存在code，则返回code，否则返回注释。
                            if p == 0 && q + mlc.1.len() == line.len() {
                                return (CodeState::Other, LineType::Comment);
                            } else {
                                return (CodeState::Other, LineType::Code);
                            }
                        } else {
                            // 不包含多行注释结束，则进入多行注释状态。
                            if p == 0 {
                                // 多行注释在开头，则该行为注释
                                return (CodeState::InMultilineComent, LineType::Comment);
                            } else {
                                // 如果多行注释开始并不在开头，则该行为代码。
                                return (CodeState::InMultilineComent, LineType::Code);
                            }
                        }
                    }
                }
                // 判断是否存在原始字符串，避免误判。
                for ele in self.verbatim_quotes() {
                    if line.contains(ele.0) && !line.ends_with(ele.1) {
                        return (CodeState::InVerbatimQuote, LineType::Code);
                    }
                }
                (CodeState::Other, LineType::Code)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::LanguageType;

    #[test]
    fn test_regex() {
        let lt = LanguageType::Cpp;
        let s = r#"string s = "hello";"#;

        for re in lt.quotes_regex() {
            let s = re.replace(&s, "");
            assert_eq!(s, "string s = ;")
        }
    }
}
