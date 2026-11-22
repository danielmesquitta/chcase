//! Case conversion used by the `chcase` CLI.
//!
//! Most cases come from [`heck`]. The Go-style cases add Go's common
//! initialisms on top of heck's word splitting, and the plain
//! lowercase/uppercase cases keep the words separated by single spaces.

mod go;

use std::io::{self, BufRead, Write};

use heck::{
    ToKebabCase, ToLowerCamelCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase, ToTitleCase,
    ToTrainCase, ToUpperCamelCase,
};

/// A target case.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Case {
    /// `helloWorld`
    Camel,
    /// `helloWorld`, with Go initialisms: `userID`
    GoCamel,
    /// `HelloWorld`
    Pascal,
    /// `HelloWorld`, with Go initialisms: `UserID`
    GoPascal,
    /// `hello_world`
    Snake,
    /// `HELLO_WORLD`
    UpperSnake,
    /// `hello-world`
    Kebab,
    /// `HELLO-WORLD`
    UpperKebab,
    /// `Hello World`
    Title,
    /// `Hello-World`
    Train,
    /// `hello world`
    Lower,
    /// `HELLO WORLD`
    Upper,
}

/// Converts `text` to `case`.
pub fn convert(text: &str, case: Case) -> String {
    match case {
        Case::Camel => text.to_lower_camel_case(),
        Case::GoCamel => go::camel(text),
        Case::Pascal => text.to_upper_camel_case(),
        Case::GoPascal => go::pascal(text),
        Case::Snake => text.to_snake_case(),
        Case::UpperSnake => text.to_shouty_snake_case(),
        Case::Kebab => text.to_kebab_case(),
        Case::UpperKebab => text.to_shouty_kebab_case(),
        Case::Title => text.to_title_case(),
        Case::Train => text.to_train_case(),
        Case::Lower => words(text).to_lowercase(),
        Case::Upper => words(text).to_uppercase(),
    }
}

/// Converts every line of `input` to `case`, writing one line per input line.
///
/// Blank lines are kept, `\r\n` endings are accepted, and invalid UTF-8 is
/// replaced with U+FFFD (a word boundary) instead of failing.
pub fn convert_lines(
    mut input: impl BufRead,
    mut output: impl Write,
    case: Case,
) -> io::Result<()> {
    let mut buf = Vec::new();
    while input.read_until(b'\n', &mut buf)? != 0 {
        let line = buf.strip_suffix(b"\n").unwrap_or(&buf);
        let line = line.strip_suffix(b"\r").unwrap_or(line);
        writeln!(output, "{}", convert(&String::from_utf8_lossy(line), case))?;
        buf.clear();
    }
    output.flush()
}

/// Splits on anything that is not a letter or digit (the same rule heck uses)
/// and rejoins the words with single spaces.
fn words(text: &str) -> String {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::spaces("hello world", "helloWorld")]
    #[case::pascal_input("HelloWorld", "helloWorld")]
    #[case::acronym("HTTPServer url", "httpServerUrl")]
    #[case::punctuation("Hello, World!", "helloWorld")]
    #[case::delimiters("foo_bar-baz.qux", "fooBarBazQux")]
    #[case::symbols("a+b=c $x", "aBCX")]
    #[case::digits("port80 utf8Value", "port80Utf8Value")]
    #[case::accents("ça va École", "çaVaÉcole")]
    #[case::trimmed("  lead trail  ", "leadTrail")]
    #[case::empty("", "")]
    #[case::only_punctuation("!!!", "")]
    fn camel(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(convert(input, Case::Camel), expected);
    }

    #[rstest]
    #[case::spaces("hello world", "HelloWorld")]
    #[case::camel_input("helloWorld", "HelloWorld")]
    #[case::acronym("HTTPServer url", "HttpServerUrl")]
    #[case::initialism_not_kept("user id", "UserId")]
    #[case::accents("ça va", "ÇaVa")]
    #[case::empty("", "")]
    fn pascal(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(convert(input, Case::Pascal), expected);
    }

    #[rstest]
    #[case::spaces("hello world", "hello_world")]
    #[case::pascal_input("HelloWorldExample", "hello_world_example")]
    #[case::acronym("HTTPServer url", "http_server_url")]
    #[case::kebab_input("foo-bar.baz", "foo_bar_baz")]
    #[case::symbols("price$100", "price_100")]
    #[case::version("v4.3", "v4_3")]
    #[case::greek_final_sigma("ΟΔΟΣ", "οδος")]
    #[case::empty("", "")]
    fn snake(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(convert(input, Case::Snake), expected);
    }

    #[rstest]
    #[case::spaces("hello world", "HELLO_WORLD")]
    #[case::camel_input("helloWorld", "HELLO_WORLD")]
    #[case::sharp_s("straße", "STRASSE")]
    fn upper_snake(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(convert(input, Case::UpperSnake), expected);
    }

    #[rstest]
    #[case::spaces("hello world", "hello-world")]
    #[case::pascal_input("HelloWorldExample", "hello-world-example")]
    #[case::snake_input("foo_bar", "foo-bar")]
    fn kebab(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(convert(input, Case::Kebab), expected);
    }

    #[rstest]
    #[case::spaces("hello world", "HELLO-WORLD")]
    #[case::snake_input("foo_bar", "FOO-BAR")]
    fn upper_kebab(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(convert(input, Case::UpperKebab), expected);
    }

    #[rstest]
    #[case::spaces("hello world", "Hello World")]
    #[case::acronym("HTTPServer url", "Http Server Url")]
    #[case::snake_input("user_id", "User Id")]
    fn title(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(convert(input, Case::Title), expected);
    }

    #[rstest]
    #[case::spaces("hello world", "Hello-World")]
    #[case::camel_input("helloWorld", "Hello-World")]
    fn train(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(convert(input, Case::Train), expected);
    }

    #[rstest]
    #[case::words("Hello World", "hello world")]
    #[case::no_case_split("HelloWorld", "helloworld")]
    #[case::punctuation_trimmed("Hello, World!", "hello world")]
    #[case::delimiters("foo_bar-baz.qux", "foo bar baz qux")]
    #[case::surrounding_space("  lead   trail  ", "lead trail")]
    #[case::only_punctuation("!!!", "")]
    #[case::greek_final_sigma("ΟΔΟΣ", "οδος")]
    fn lower(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(convert(input, Case::Lower), expected);
    }

    #[rstest]
    #[case::words("Hello World", "HELLO WORLD")]
    #[case::punctuation_trimmed("Hello, World!", "HELLO WORLD")]
    #[case::digits("v4.3 port80", "V4 3 PORT80")]
    #[case::sharp_s("straße", "STRASSE")]
    fn upper(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(convert(input, Case::Upper), expected);
    }

    #[rstest]
    #[case::one_line("hello world\n", "hello_world\n")]
    #[case::many_lines("hello world\nfoo bar\n", "hello_world\nfoo_bar\n")]
    #[case::no_trailing_newline("hello world", "hello_world\n")]
    #[case::crlf("hello world\r\nfoo bar\r\n", "hello_world\nfoo_bar\n")]
    #[case::blank_lines_kept("a b\n\nc d\n", "a_b\n\nc_d\n")]
    #[case::empty_input("", "")]
    fn lines(#[case] input: &str, #[case] expected: &str) {
        let mut output = Vec::new();
        convert_lines(input.as_bytes(), &mut output, Case::Snake).unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), expected);
    }

    /// Invalid bytes become U+FFFD, which is not a letter, so it splits words.
    #[test]
    fn lines_tolerate_invalid_utf8() {
        let mut output = Vec::new();
        convert_lines(&b"caf\xe9 au lait\n"[..], &mut output, Case::Snake).unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "caf_au_lait\n");
    }
}
