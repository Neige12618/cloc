use serde::Deserialize;

#[derive(Deserialize)]
pub struct Language {
    pub name: String,
    #[serde(default)]
    pub line_comment: Vec<String>,
    #[serde(default)]
    pub multi_line_comments: Vec<(String, String)>,
    #[serde(default)]
    pub quotes: Vec<(String, String)>,
    #[serde(default)]
    pub verbatim_quotes: Vec<(String, String)>,
    pub extensions: Vec<String>,
}

#[derive(Deserialize)]
pub struct Languages {
    pub languages: Vec<Language>,
}

pub fn parse_json(path: &str) -> Result<Languages, Box<dyn std::error::Error>> {
    let json_str = std::fs::read_to_string(path)?;
    let data: Languages = serde_json::from_str(&json_str)?;
    Ok(data)
}
