//! Go-style camelCase and PascalCase, which keep Go's common initialisms in
//! upper case (`userID`, `HTTPServer`).

use heck::ToSnakeCase;

/// golint's common initialisms.
const INITIALISMS: [&str; 38] = [
    "ACL", "API", "ASCII", "CPU", "CSS", "DNS", "EOF", "GUID", "HTML", "HTTP", "HTTPS", "ID", "IP",
    "JSON", "LHS", "QPS", "RAM", "RHS", "RPC", "SLA", "SMTP", "SQL", "SSH", "TCP", "TLS", "TTL",
    "UDP", "UI", "UID", "URI", "URL", "UTF8", "UUID", "VM", "XML", "XMPP", "XSRF", "XSS",
];

/// camelCase with initialisms; the first word stays lower case even when it
/// is an initialism (`jsonData`), as Go expects for unexported names.
pub fn camel(text: &str) -> String {
    join(text, true)
}

/// PascalCase with initialisms (`JSONData`).
pub fn pascal(text: &str) -> String {
    join(text, false)
}

fn join(text: &str, lower_first: bool) -> String {
    // heck's snake case gives its word split, already lower cased.
    let snake = text.to_snake_case();
    let mut words = snake.split('_');
    let mut out = String::with_capacity(snake.len());
    if lower_first {
        out.extend(words.next());
    }
    for word in words {
        push_capitalized(&mut out, word);
    }
    out
}

/// Appends `word` as an initialism if it is one, with its first letter in
/// upper case otherwise.
fn push_capitalized(out: &mut String, word: &str) {
    if let Some(initialism) = INITIALISMS.iter().find(|i| i.eq_ignore_ascii_case(word)) {
        out.push_str(initialism);
        return;
    }
    let mut chars = word.chars();
    if let Some(first) = chars.next() {
        out.extend(first.to_uppercase());
        out.push_str(chars.as_str());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Case, convert};
    use rstest::rstest;

    #[rstest]
    #[case::initialism_last("user id", "userID")]
    #[case::initialisms_in_middle("HTTPServer url", "httpServerURL")]
    #[case::first_word_stays_lower("json data", "jsonData")]
    #[case::first_word_upper_input("ID user", "idUser")]
    #[case::digit_initialism("to utf8", "toUTF8")]
    #[case::plural_is_not_initialism("user ids", "userIds")]
    #[case::camel_input("userId", "userID")]
    #[case::single_word("json", "json")]
    #[case::empty("", "")]
    fn go_camel(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(camel(input), expected);
    }

    #[rstest]
    #[case::initialism_last("user id", "UserID")]
    #[case::initialisms("HTTPServer url", "HTTPServerURL")]
    #[case::first_word_initialism("json data", "JSONData")]
    #[case::several("utf8 value https id ids", "UTF8ValueHTTPSIDIds")]
    #[case::single_word("json", "JSON")]
    #[case::accents("école id", "ÉcoleID")]
    #[case::dotless_i_is_not_initialism("user ıd", "UserId")]
    #[case::long_s_is_not_initialism("ſql query", "SqlQuery")]
    #[case::empty("", "")]
    fn go_pascal(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(pascal(input), expected);
    }

    /// Without initialisms, the Go cases must match heck's.
    #[rstest]
    #[case("hello world")]
    #[case("HelloWorldExample")]
    #[case("foo_bar-baz.qux")]
    #[case("v4.3 port80 200status")]
    #[case("ça va École ÉCOLE")]
    #[case("straße ǆ")]
    #[case("ΟΔΟΣ")]
    #[case("  lead trail  ")]
    fn matches_heck_without_initialisms(#[case] input: &str) {
        assert_eq!(camel(input), convert(input, Case::Camel));
        assert_eq!(pascal(input), convert(input, Case::Pascal));
    }
}
