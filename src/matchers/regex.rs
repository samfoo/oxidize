use super::Matcher;
use regex::Regex;

pub struct MatchesRegex<'a>(pub &'a str);

impl<'a> Matcher<&'a str> for MatchesRegex<'a> {
    fn matches(&self, lhs: &&'a str) -> bool {
        match Regex::new(self.0) {
            Ok(re) => re.is_match(lhs),
            Err(_) => false
        }
    }

    fn fail_msg(&self, lhs: &&'a str) -> String {
        format!("expected {:?} to match {:?}", lhs, self.0)
    }

    fn negated_fail_msg(&self, lhs: &&'a str) -> String {
        format!("expected {:?} not to match {:?}", lhs, self.0)
    }
}

impl<'a> Matcher<String> for MatchesRegex<'a> {
    fn matches(&self, lhs: &String) -> bool {
        match Regex::new(self.0) {
            Ok(re) => re.is_match(&*lhs),
            Err(_) => false
        }
    }

    fn fail_msg(&self, lhs: &String) -> String {
        format!("expected {:?} to match {:?}", lhs, self.0)
    }

    fn negated_fail_msg(&self, lhs: &String) -> String {
        format!("expected {:?} not to match {:?}", lhs, self.0)
    }
}

#[cfg(test)]
mod test {
    mod str_match_regex {
        use super::super::super::super::dsl::*;

        #[test]
        fn test_match_regex_matches() {
            expect("Hello").to(match_regex(r"..ll."))
        }

        #[test]
        #[should_panic(expected="expected \"Hello\" to match \".\\\\o/.\"")]
        fn test_match_regex_fails_with_message() {
            expect("Hello").to(match_regex(".\\o/."))
        }

        #[test]
        #[should_panic(expected="expected \"-\\\\o/-\" not to match \".....\"")]
        fn test_negated_match_regex_fails_with_message() {
            expect("-\\o/-").to_not(match_regex("....."))
        }
    }

    mod string_match_regex {
        use super::super::super::super::dsl::*;

        #[test]
        fn test_match_regex_matches() {
            expect("Hello".to_string()).to(match_regex(r"..ll."))
        }

        #[test]
        #[should_panic(expected="expected \"Hello\" to match \".\\\\o/.\"")]
        fn test_match_regex_fails_with_message() {
            expect("Hello".to_string()).to(match_regex(".\\o/."))
        }

        #[test]
        #[should_panic(expected="expected \"-\\\\o/-\" not to match \".....\"")]
        fn test_negated_match_regex_fails_with_message() {
            expect("-\\o/-".to_string()).to_not(match_regex("....."))
        }
    }
}
