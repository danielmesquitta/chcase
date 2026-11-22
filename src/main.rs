use std::io::{self, IsTerminal, Write};
use std::process::ExitCode;

use chcase::{Case, convert, convert_lines};
use clap::error::ErrorKind;
use clap::{Args, CommandFactory, FromArgMatches, Parser, Subcommand};
use clap_complete::Shell;

const EXAMPLES: &str = "\
Examples:
  chcase camel \"hello world\"
  chcase snake HelloWorld
  echo \"hello world\" | chcase pascal
  chcase kebab < names.txt";

/// Change text between camelCase, snake_case, kebab-case and other cases.
#[derive(Parser)]
// A fixed bin_name keeps usage lines free of `.exe` on Windows.
#[command(
    bin_name = "chcase",
    version,
    about,
    after_help = EXAMPLES,
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Convert to camelCase (hello world -> helloWorld)
    Camel(Input),
    /// Convert to camelCase with Go initialisms (user id -> userID)
    #[command(name = "gocamel")]
    GoCamel(Input),
    /// Convert to PascalCase (hello world -> HelloWorld)
    Pascal(Input),
    /// Convert to PascalCase with Go initialisms (user id -> UserID)
    #[command(name = "gopascal")]
    GoPascal(Input),
    /// Convert to snake_case (hello world -> hello_world)
    Snake(Input),
    /// Convert to UPPER_SNAKE_CASE (hello world -> HELLO_WORLD)
    #[command(name = "uppersnake")]
    UpperSnake(Input),
    /// Convert to kebab-case (hello world -> hello-world)
    Kebab(Input),
    /// Convert to UPPER-KEBAB-CASE (hello world -> HELLO-WORLD)
    #[command(name = "upperkebab")]
    UpperKebab(Input),
    /// Convert to Title Case (hello world -> Hello World)
    Title(Input),
    /// Convert to Train-Case (hello world -> Hello-World)
    Train(Input),
    /// Convert to lowercase words (Hello, World! -> hello world)
    Lowercase(Input),
    /// Convert to UPPERCASE words (Hello, World! -> HELLO WORLD)
    Uppercase(Input),
    /// Print a shell completion script
    Completion {
        /// Shell to generate the script for
        shell: Shell,
    },
}

#[derive(Args)]
struct Input {
    /// Text to convert, joined with spaces. Without it, each line of stdin
    /// is converted.
    text: Vec<String>,
}

fn main() -> ExitCode {
    let mut cmd = Cli::command();
    let matches = cmd.get_matches_mut();
    let cli = Cli::from_arg_matches(&matches).unwrap_or_else(|err| err.exit());
    let (case, input) = match cli.command {
        Command::Camel(input) => (Case::Camel, input),
        Command::GoCamel(input) => (Case::GoCamel, input),
        Command::Pascal(input) => (Case::Pascal, input),
        Command::GoPascal(input) => (Case::GoPascal, input),
        Command::Snake(input) => (Case::Snake, input),
        Command::UpperSnake(input) => (Case::UpperSnake, input),
        Command::Kebab(input) => (Case::Kebab, input),
        Command::UpperKebab(input) => (Case::UpperKebab, input),
        Command::Title(input) => (Case::Title, input),
        Command::Train(input) => (Case::Train, input),
        Command::Lowercase(input) => (Case::Lower, input),
        Command::Uppercase(input) => (Case::Upper, input),
        Command::Completion { shell } => {
            let name = cmd.get_name().to_owned();
            clap_complete::generate(shell, &mut cmd, name, &mut io::stdout());
            return ExitCode::SUCCESS;
        }
    };

    let result = if !input.text.is_empty() {
        writeln!(io::stdout(), "{}", convert(&input.text.join(" "), case))
    } else if io::stdin().is_terminal() {
        // Report against the subcommand so the usage line shows its arguments.
        let name = matches.subcommand_name().expect("a subcommand is required");
        cmd.find_subcommand_mut(name)
            .expect("parsed subcommand exists")
            .error(
                ErrorKind::MissingRequiredArgument,
                "no input: pass TEXT as arguments or pipe it via stdin",
            )
            .exit()
    } else {
        convert_lines(io::stdin().lock(), io::stdout().lock(), case)
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        // The reader went away (`chcase snake < big.txt | head`): not an error.
        Err(err) if err.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}
