use clap::ValueEnum;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::state::{CodeState, LineType};

#[derive(PartialEq, Clone, Copy, ValueEnum)]
pub enum LanguageType {
    Cpp,
    Ruby,
}

static CPP_QUOTE_RE: Lazy<Vec<Regex>> = Lazy::new(|| {
    LanguageType::Cpp
        .quotes()
        .iter()
        .map(|q| Regex::new(&format!("{}(.*?){}", q.0, q.1)).unwrap())
        .collect()
});
static RUBY_QUOTE_RE: Lazy<Vec<Regex>> = Lazy::new(|| {
    LanguageType::Ruby
        .quotes()
        .iter()
        .map(|q| Regex::new(&format!("{}(.*?){}", q.0, q.1)).unwrap())
        .collect()
});

type StaticStr = &'static str;

impl LanguageType {
    pub fn line_comment(&self) -> &'static [StaticStr] {
        match self {
            LanguageType::Cpp => &["//"],
            LanguageType::Ruby => &["#"],
        }
    }

    pub fn multi_line_comments(&self) -> &'static [(StaticStr, StaticStr)] {
        match self {
            LanguageType::Cpp => &[("/*", "*/")],
            LanguageType::Ruby => &[("=begin", "=end")],
        }
    }

    pub fn quotes(&self) -> &'static [(StaticStr, StaticStr)] {
        match self {
            LanguageType::Cpp => &[("\"", "\"")],
            LanguageType::Ruby => &[("\"", "\""), ("'", "'")],
        }
    }

    pub fn quotes_regex(&self) -> &'static Vec<Regex> {
        match self {
            LanguageType::Cpp => &CPP_QUOTE_RE,
            LanguageType::Ruby => &RUBY_QUOTE_RE,
        }
    }

    pub fn verbatim_quotes(&self) -> &'static [(StaticStr, StaticStr)] {
        match self {
            LanguageType::Cpp => &[("R\"(", ")\"")],
            LanguageType::Ruby => &[],
        }
    }

    pub fn from_file_extension(extension: &str) -> Option<Self> {
        match extension {
            "cc" | "cpp" | "cxx" | "c++" => Some(LanguageType::Cpp),
            "rb" => Some(LanguageType::Ruby),
            _ => None,
        }
    }
}

impl LanguageType {
    pub fn parse_line(&self, line: &str, prev: CodeState) -> (CodeState, LineType) {
        let line = line.trim();
        if line.is_empty() {
            return (prev, LineType::Blank);
        }

        match prev {
            CodeState::InMultilineComent => {
                for mlc in self.multi_line_comments() {
                    if line.contains(mlc.1) {
                        if line.ends_with(mlc.1) {
                            return (CodeState::Other, LineType::Comment);
                        } else {
                            return (CodeState::Other, LineType::Code);
                        }
                    }
                }
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
                let regex = self.quotes_regex();
                let mut line = line.to_string();
                for re in regex {
                    line = re.replace_all(&line, "\"\"").to_string();
                }
                if self.line_comment().iter().any(|s| line.starts_with(s)) {
                    return (CodeState::Other, LineType::Comment);
                }
                for mlc in self.multi_line_comments() {
                    if let Some(p) = line.find(mlc.0) {
                        if let Some(q) = line.find(mlc.1) {
                            if p == 0 && q + mlc.1.len() == line.len() {
                                return (CodeState::Other, LineType::Comment);
                            } else {
                                return (CodeState::Other, LineType::Code);
                            }
                        } else {
                            if p == 0 {
                                return (CodeState::InMultilineComent, LineType::Comment);
                            } else {
                                return (CodeState::InMultilineComent, LineType::Code);
                            }
                        }
                    }
                }
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
