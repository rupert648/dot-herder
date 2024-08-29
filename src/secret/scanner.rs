use colored::*;
use regex::Regex;
use std::collections::HashSet;

pub struct ScanResult {
    pub line_number: usize,
    pub culprit_line: String,
    pub prev_line: Option<String>,
    pub next_line: Option<String>,
    pub reason: String,
}

pub struct Scanner {
    _keywords: HashSet<String>,
    keyword_regex: Regex,
}

impl Scanner {
    pub fn new() -> Self {
        let keywords = vec![
            "apikey",
            "api_key",
            "password",
            "username",
            "secret",
            "token",
            "auth",
            "credential",
            "private_key",
            "privkey",
            "access_key",
            "npm_token",
        ];
        let keyword_pattern = keywords.join("|");
        let keyword_regex = Regex::new(&format!(r"(?i)({})", keyword_pattern)).unwrap();

        Scanner {
            _keywords: keywords.into_iter().map(String::from).collect(),
            keyword_regex,
        }
    }

    pub fn scan(&self, content: &str) -> Vec<ScanResult> {
        let lines: Vec<&str> = content.lines().collect();
        let mut results = Vec::new();

        for (i, line) in lines.iter().enumerate() {
            if self.keyword_regex.is_match(line) {
                results.push(ScanResult {
                    line_number: i + 1,
                    culprit_line: line.to_string(),
                    prev_line: if i > 0 {
                        Some(lines[i - 1].to_string())
                    } else {
                        None
                    },
                    next_line: lines.get(i + 1).map(|&s| s.to_string()),
                    reason: "Keyword match".to_string(),
                });
            }
        }

        results
    }
}

impl ScanResult {
    pub fn print_formatted(&self) {
        println!("{}", "━".repeat(80).bright_black());
        println!("{}: {}", "Reason".bright_red().bold(), self.reason);
        println!("{}", "━".repeat(80).bright_black());

        let line_number_width = self.line_number.to_string().len();

        if let Some(prev_line) = &self.prev_line {
            print_line(self.line_number - 1, prev_line, line_number_width, false);
        }

        print_line(
            self.line_number,
            &self.culprit_line,
            line_number_width,
            true,
        );

        if let Some(next_line) = &self.next_line {
            print_line(self.line_number + 1, next_line, line_number_width, false);
        }

        println!();
    }
}

fn print_line(line_number: usize, content: &str, width: usize, is_culprit: bool) {
    let line_num = format!("{:>width$}", line_number, width = width).bright_blue();
    let separator = "│".bright_blue();
    let content = if is_culprit {
        content.bright_white()
    } else {
        content.bright_black()
    };
    println!("{} {} {}", line_num, separator, content);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_scanner() {
        let scanner = Scanner::new();
        let content = r#"
This is a normal line
Here's a line with a password: mySecretPass123
Another normal line
ApiKey: abcdefghijklmnop
Last line
"#;

        let results = scanner.scan(content);

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].line_number, 3);
        assert_eq!(results[0].reason, "Keyword match");
        assert_eq!(results[1].line_number, 5);
        assert_eq!(results[1].reason, "Keyword match");
    }

    #[test]
    fn test_secret_is_present() {
        let scanner = Scanner::new();
        let content = "This is a normal line\nHere's a secret: myApiKey123\nAnother normal line";
        let results = scanner.scan(content);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].line_number, 2);
        assert_eq!(results[0].reason, "Keyword match");
        assert!(results[0].culprit_line.contains("myApiKey123"));
    }

    #[test]
    fn test_no_secret_present() {
        let scanner = Scanner::new();
        let content = "This is a normal line\nNothing here\nJust regular text";
        let results = scanner.scan(content);

        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_multiple_same_secret_present() {
        let scanner = Scanner::new();
        let content = "Password: 123456\nAnother password: 123456\nYet another password: 123456";
        let results = scanner.scan(content);

        assert_eq!(results.len(), 3);
        for result in &results {
            assert_eq!(result.reason, "Keyword match");
            assert!(result.culprit_line.to_lowercase().contains("password"));
        }
    }

    #[test]
    fn test_multiple_different_secret_present() {
        let scanner = Scanner::new();
        let content = "API_KEY: abcdef\nUsername: john_doe\nPassword: secret123";
        let results = scanner.scan(content);

        assert_eq!(results.len(), 3);
        assert!(results.iter().any(|r| r.culprit_line.contains("API_KEY")));
        assert!(results.iter().any(|r| r.culprit_line.contains("Username")));
        assert!(results.iter().any(|r| r.culprit_line.contains("Password")));
    }
}
