use assert_cmd::Command;
use predicates::prelude::*;
use rstest::rstest;

fn chcase() -> Command {
    assert_cmd::cargo::cargo_bin_cmd!("chcase")
}

#[rstest]
#[case::camel("camel", "helloWorldExample")]
#[case::gocamel("gocamel", "helloWorldExample")]
#[case::pascal("pascal", "HelloWorldExample")]
#[case::gopascal("gopascal", "HelloWorldExample")]
#[case::snake("snake", "hello_world_example")]
#[case::uppersnake("uppersnake", "HELLO_WORLD_EXAMPLE")]
#[case::kebab("kebab", "hello-world-example")]
#[case::upperkebab("upperkebab", "HELLO-WORLD-EXAMPLE")]
#[case::title("title", "Hello World Example")]
#[case::train("train", "Hello-World-Example")]
#[case::lowercase("lowercase", "hello world example")]
#[case::uppercase("uppercase", "HELLO WORLD EXAMPLE")]
fn every_command_converts_its_argument(#[case] command: &str, #[case] expected: &str) {
    chcase()
        .args([command, "hello world example"])
        .assert()
        .success()
        .stdout(format!("{expected}\n"));
}

#[rstest]
#[case::one_quoted(&["hello world"], "hello_world\n")]
#[case::several_words(&["Hello", "World", "Foo Bar"], "hello_world_foo_bar\n")]
#[case::empty_string(&[""], "\n")]
#[case::after_double_dash(&["--", "-hello", "world"], "hello_world\n")]
fn arguments_are_joined(#[case] text: &[&str], #[case] expected: &str) {
    chcase()
        .arg("snake")
        .args(text)
        .assert()
        .success()
        .stdout(expected.to_owned());
}

#[rstest]
#[case::one_line("hello world\n", "hello_world\n")]
#[case::each_line("hello world\nfoo bar\nbaz qux\n", "hello_world\nfoo_bar\nbaz_qux\n")]
#[case::no_trailing_newline("hello world", "hello_world\n")]
#[case::crlf("hello world\r\nfoo bar\r\n", "hello_world\nfoo_bar\n")]
#[case::blank_lines_kept("a b\n\nc d\n", "a_b\n\nc_d\n")]
#[case::empty("", "")]
fn stdin_is_converted_line_by_line(#[case] input: &str, #[case] expected: &str) {
    chcase()
        .arg("snake")
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected.to_owned());
}

#[test]
fn arguments_take_precedence_over_stdin() {
    chcase()
        .args(["camel", "from args"])
        .write_stdin("from stdin\n")
        .assert()
        .success()
        .stdout("fromArgs\n");
}

#[rstest]
#[case::no_command(&[], "Usage: chcase <COMMAND>")]
#[case::unknown_command(&["camle", "x"], "unrecognized subcommand 'camle'")]
#[case::unknown_flag(&["camel", "-x"], "unexpected argument '-x'")]
#[case::bad_shell(&["completion", "tcsh"], "invalid value 'tcsh'")]
fn usage_errors_exit_with_2(#[case] args: &[&str], #[case] message: &str) {
    chcase()
        .args(args)
        .assert()
        .code(2)
        .stderr(predicate::str::contains(message));
}

#[rstest]
#[case::bash("bash", "_chcase()")]
#[case::zsh("zsh", "#compdef chcase")]
#[case::fish("fish", "complete -c chcase")]
#[case::powershell("powershell", "Register-ArgumentCompleter")]
#[case::elvish("elvish", "set edit:completion:arg-completer[chcase]")]
fn completion_scripts_are_generated(#[case] shell: &str, #[case] marker: &str) {
    chcase()
        .args(["completion", shell])
        .assert()
        .success()
        .stdout(predicate::str::contains(marker));
}

#[test]
fn version_is_printed() {
    chcase()
        .arg("--version")
        .assert()
        .success()
        .stdout(format!("chcase {}\n", env!("CARGO_PKG_VERSION")));
}
