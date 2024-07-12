#[derive(PartialEq)]
pub enum LanguageType {
    Cpp,
    Ruby,
}

type StaticStr = &'static str;

impl LanguageType {
    pub fn name(&self) -> StaticStr {
        match self {
            LanguageType::Cpp => "C++",
            LanguageType::Ruby => "Ruby",
        }
    }

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

impl std::str::FromStr for LanguageType {
    type Err = &'static str;

    fn from_str(from: &str) -> Result<Self, Self::Err> {
        match &*from.to_lowercase() {
            "cpp" => Ok(LanguageType::Cpp),
            "ruby" => Ok(LanguageType::Ruby),
            _ => Err("Unknown language type"),
        }
    }
}

impl std::fmt::Display for LanguageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
