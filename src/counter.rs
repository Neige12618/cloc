use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::{
    language_type::LanguageType,
    state::{CodeState, LineType},
};

#[derive(Debug)]
pub struct Counter {
    pub files: usize,
    pub comments: usize,
    pub blanks: usize,
    pub code: usize,
}

impl std::ops::Add for Counter {
    type Output = Counter;

    fn add(self, rhs: Counter) -> Counter {
        Counter {
            files: self.files + rhs.files,
            comments: self.comments + rhs.comments,
            blanks: self.blanks + rhs.blanks,
            code: self.code + rhs.code,
        }
    }
}

impl Counter {
    /// 创建一个新的Counter，一个新的Counter会自带一个文件。
    pub fn new() -> Counter {
        Counter {
            files: 1,
            comments: 0,
            blanks: 0,
            code: 0,
        }
    }

    /// 创建一个空的Counter，不计文件数。
    pub fn none() -> Counter {
        Counter {
            files: 0,
            comments: 0,
            blanks: 0,
            code: 0,
        }
    }

    /// 计算所有行数。
    pub fn lines(&self) -> usize {
        self.comments + self.blanks + self.code
    }

    /// 根据LineType增加计数。
    pub fn add(&mut self, state: &LineType) {
        match *state {
            LineType::Blank => self.blanks += 1,
            LineType::Comment => self.comments += 1,
            LineType::Code => self.code += 1,
        }
    }
}

pub fn count_lines(path: &PathBuf, language_type: LanguageType) -> Counter {
    // 打开文件，如果无法打开，则只计算文件，不计算代码行数。
    // 可能因为权限不够无法打开文件，文件路径是必定正确的。
    let file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(_) => return Counter::new(),
    };
    let reader = std::io::BufReader::new(file);
    // 如果无法读取，比如内容是二进制，则只计算文件，不计算代码行数。
    count_lines_from_reader(reader, language_type).unwrap_or(Counter::new())
}

pub fn count_lines_from_reader<R>(
    reader: BufReader<R>,
    language_type: LanguageType,
) -> std::io::Result<Counter>
where
    R: std::io::Read,
{
    let mut counts = Counter::new();
    let mut prev = CodeState::Other;
    let mut line_type;

    for line in reader.lines() {
        (prev, line_type) = language_type.parse_line(&line?, prev);
        counts.add(&line_type);
    }

    Ok(counts)
}

#[cfg(test)]
mod test {
    use std::io::{BufReader, Cursor};

    use crate::counter::count_lines_from_reader;

    #[test]
    fn test_cpp_count() {
        let s = r#"
        int main() {
            /*
             */
            return 0;
            return /**/ 1;  // dadwf
            char* poem = R"(
              // This is not a comment.
            /* Not comment!
               Again not comment. */
            )";
            char* s =
                "//hell"
                "/*hello"
                ""
                "dwaf*/";
        }"#;
        let counts = count_lines_from_reader(
            BufReader::new(Cursor::new(s)),
            crate::language_type::LanguageType::Cpp,
        )
        .unwrap();
        assert_eq!(counts.code, 14);
        assert_eq!(counts.comments, 2);
        assert_eq!(counts.blanks, 1);
    }

    #[test]
    fn test_ruby_count() {
        let s = r#"
module Enumerable
  class << self
  =begin
     Provides the cross-product of two or more Enumerables.
     This is the class-level method. The instance method
     calls on this.
    
       Enumerable.cart([1,2], [4], ["apple", "banana"])
       => [[1, 4, "apple"], [1, 4, "banana"], [2, 4, "apple"], [2, 4, "banana"]]
    
       Enumerable.cart([1,2], [3,4])
       => [[1, 3], [1, 4], [2, 3], [2, 4]]
  =end
"#;
        let counts = count_lines_from_reader(
            BufReader::new(Cursor::new(s)),
            crate::language_type::LanguageType::Ruby,
        )
        .unwrap();
        assert_eq!(counts.code, 2);
        assert_eq!(counts.comments, 9);
        assert_eq!(counts.blanks, 3);
    }
}
