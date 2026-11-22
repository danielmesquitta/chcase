# chcase

Change text between camelCase, snake_case, kebab-case and other cases, from
arguments or line by line from stdin.

```console
$ chcase snake "Hello World"
hello_world

$ printf 'user id\nhttp server\n' | chcase gopascal
UserID
HTTPServer
```

## Installation

```bash
cargo install --git https://github.com/danielmesquitta/chcase --tag v0.1.1
```

Drop `--tag` to install the latest `main` instead of a release.

Or build from source:

```bash
git clone https://github.com/danielmesquitta/chcase
cd chcase
cargo install --path .
```

## Usage

```bash
chcase <COMMAND> [TEXT]...
```

- Arguments are joined with spaces and converted as one line.
- Without arguments, each line of stdin is converted separately. Blank lines
  are kept, and `\r\n` line endings are accepted.
- Use `--` before text that starts with a dash: `chcase snake -- -flag-name`.

### Commands

| Command      | Output               | `hello world` becomes |
|--------------|----------------------|-----------------------|
| `camel`      | camelCase            | `helloWorld`          |
| `pascal`     | PascalCase           | `HelloWorld`          |
| `snake`      | snake_case           | `hello_world`         |
| `uppersnake` | UPPER_SNAKE_CASE     | `HELLO_WORLD`         |
| `kebab`      | kebab-case           | `hello-world`         |
| `upperkebab` | UPPER-KEBAB-CASE     | `HELLO-WORLD`         |
| `title`      | Title Case           | `Hello World`         |
| `train`      | Train-Case           | `Hello-World`         |
| `lowercase`  | lowercase words      | `hello world`         |
| `uppercase`  | UPPERCASE words      | `HELLO WORLD`         |

Input can be in any case. Words are split on spaces, punctuation and symbols,
and on case changes (`HTTPServer` is `HTTP` + `Server`):

```console
$ chcase kebab "HTTPServer_config.json"
http-server-config-json
```

`lowercase` and `uppercase` only change case: they split on punctuation and
symbols but not on case changes, and join the words with single spaces.

```console
$ chcase lowercase "  Hello, World! "
hello world
```

### Go-style commands

`gocamel` and `gopascal` keep Go's common initialisms (`ID`, `URL`, `HTTP`,
`JSON` and the rest of [golint's list][initialisms]) in upper case. In
`gocamel`, the first word stays lower case, as Go expects for unexported names.

| Command    | `user id` | `http server url` | `json data` |
|------------|-----------|-------------------|-------------|
| `gocamel`  | `userID`  | `httpServerURL`   | `jsonData`  |
| `gopascal` | `UserID`  | `HTTPServerURL`   | `JSONData`  |

[initialisms]: src/go.rs

### Files and pipes

```bash
# Convert every line of a file
chcase snake < names.txt

# Chain with other tools
curl -s api.example.com/data | jq -r '.[].name' | chcase pascal
```

### Shell completion

```bash
chcase completion bash > ~/.local/share/bash-completion/completions/chcase
chcase completion zsh > "${fpath[1]}/_chcase"
chcase completion fish > ~/.config/fish/completions/chcase.fish
```

PowerShell and Elvish are also supported.

### Exit codes

| Code | Meaning                                                      |
|------|--------------------------------------------------------------|
| 0    | Success                                                      |
| 1    | I/O error while reading stdin or writing stdout              |
| 2    | Usage error: unknown command or flag, or no input to convert |

## Coming from caseconv

`chcase` is a Rust rewrite of the Go tool
[caseconv](https://github.com/danielmesquitta/caseconv). The commands keep
their names, with `title` and `train` added. Some output changed:

| Input                      | caseconv           | chcase              |
|----------------------------|--------------------|---------------------|
| Multi-line stdin, `snake`  | one joined line    | one line per line   |
| `Hello, World!`, `lowercase` | `hello  world `  | `hello world`       |
| `a+b=c`, `snake`           | `abc`              | `a_b_c`             |
| `price$100`, `snake`       | `price100`         | `price_100`         |
| `200Status`, `snake`       | `200_status`       | `200status`         |
| `straße`, `uppersnake`     | `STRAßE`           | `STRASSE`           |

- Symbols such as `+`, `=` and `$` now separate words instead of being dropped.
- A digit followed by an upper-case letter no longer starts a new word.
- Upper-casing follows full Unicode rules, so one letter can become two.
- Usage errors exit with 2 instead of 1, and empty stdin prints nothing.

## Development

```bash
make check   # fmt-check, clippy with -D warnings, tests
make help    # all targets
```

## License

[MIT](LICENSE)
