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
    pub fn new() -> Counter {
        Counter {
            files: 1,
            comments: 0,
            blanks: 0,
            code: 0,
        }
    }

    pub fn none() -> Counter {
        Counter {
            files: 0,
            comments: 0,
            blanks: 0,
            code: 0,
        }
    }

    pub fn lines(&self) -> usize {
        self.comments + self.blanks + self.code
    }

    pub fn add(&mut self, state: &LineType) {
        match *state {
            LineType::Blank => self.blanks += 1,
            LineType::Comment => self.comments += 1,
            LineType::Code => self.code += 1,
        }
    }
}

pub fn count_lines(path: &PathBuf, language_type: LanguageType) -> Counter {
    let file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(_) => return Counter::new(),
    };
    let reader = std::io::BufReader::new(file);
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
    use std::{path::PathBuf, str::FromStr};

    use crate::counter::count_lines;

    #[test]
    fn test_count1() {
        let counts = count_lines(
            &PathBuf::from_str(".test/example.cpp").unwrap(),
            crate::language_type::LanguageType::Cpp,
        );
        assert_eq!(counts.code, 5);
        assert_eq!(counts.comments, 9);
        assert_eq!(counts.blanks, 3);
    }
}
