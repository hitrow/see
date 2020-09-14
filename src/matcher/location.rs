use crate::config::Force;
use crate::matcher::{replace_match_keyword, END_WORD, REGEX_WORD, START_WORD};
use globset::GlobMatcher;
use regex::Regex;

// Match location
#[derive(Debug, Clone)]
pub struct LocationMatcher(MatchMode);

#[derive(Debug, Clone)]
enum MatchMode {
    Glob(GlobMatcher),
    Regex(Regex),
    Start(String),
    End(String),
}

impl LocationMatcher {
    pub fn new(location: &str) -> Self {
        // Regex
        if let Some(raw) = replace_match_keyword(location, REGEX_WORD) {
            let reg = raw.as_str().to_regex();
            return LocationMatcher(MatchMode::Regex(reg));
        }

        // Start
        if let Some(raw) = replace_match_keyword(location, START_WORD) {
            return LocationMatcher(MatchMode::Start(raw));
        }

        // End
        if let Some(raw) = replace_match_keyword(location, END_WORD) {
            return LocationMatcher(MatchMode::End(raw));
        }

        // Glob
        let glob = location.to_glob().compile_matcher();
        LocationMatcher(MatchMode::Glob(glob))
    }

    pub fn is_match(&self, path: &str) -> bool {
        match &self.0 {
            MatchMode::Glob(glob) => glob.is_match(path),
            MatchMode::Regex(reg) => reg.is_match(path),
            MatchMode::Start(s) => path.starts_with(s),
            MatchMode::End(s) => path.ends_with(s),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create() {}

    #[test]
    fn start() {
        let matcher = LocationMatcher::new("^/test/");
        assert!(matcher.is_match("/test/a"));
        assert!(matcher.is_match("/test/a/b"));
    }

    #[test]
    fn end() {
        let matcher = LocationMatcher::new("$.png");
        assert!(matcher.is_match("/test/a.png"));
        assert!(matcher.is_match("/test/a/b.png"));
    }

    #[test]
    fn regex() {
        let matcher = LocationMatcher::new(r"~/test/.*");
        assert!(matcher.is_match("/test/a"));
        assert!(matcher.is_match("/test/a/b"));
    }

    #[test]
    fn glob() {
        let matcher = LocationMatcher::new("/test/*");
        assert!(matcher.is_match("/test/a"));
        assert!(matcher.is_match("/test/a/b"));
    }
}
