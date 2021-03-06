[![Build status on GitLab CI][gitlab-ci-master-badge]][gitlab-ci-link]
[![Newest release on crates.io][crate-version-badge]][crate-link]
[![Number of downloads on crates.io][crate-download-badge]][crate-link]
[![Project license][crate-license-badge]](LICENSE)

[crate-link]: https://crates.io/crates/wikitrans
[crate-download-badge]: https://img.shields.io/crates/d/wikitrans.svg
[crate-version-badge]: https://img.shields.io/crates/v/wikitrans.svg
[crate-license-badge]: https://img.shields.io/crates/l/wikitrans.svg
[gitlab-ci-link]: https://gitlab.com/timvisee/wikitrans/commits/master
[gitlab-ci-master-badge]: https://gitlab.com/timvisee/wikitrans/badges/master/pipeline.svg

# wikitrans
Super simple CLI tool for translating words/terms using Wikipedia.

Services such as Google Translate usually aren't accurate for translating
technical terms, this tool provides an alternative by using the Wikipedia
database with user defined translations.

[![wikitrans usage demo][usage-demo-svg]][usage-demo-asciinema]  
_No demo visible here? View it on [asciinema][usage-demo-asciinema]._

This tool uses [`skim`][skim] as interactive selection interface.

## Usage
Using this tool is stupidly simple:
```bash
# Translate term with interactive language selection
wikitrans rust

# Translate term with specified languages
wikitrans rust --language en --translate nl rust
# or
wikitrans rust -l en -t nl rust
```

## Installation
To install this tool, clone the repository and install it with `cargo`.

First make sure you meet the following build requirements:
- Rust `v1.31` or higher (with [`cargo`][cargo], install using [`rustup`][rustup])
- [`git`][git]

Then use the following commands:
```bash
# Clone the repository
git clone https://gitlab.com/timvisee/wikitrans.git

# Compile and install wikitrans
cargo install --path wikitrans

# To update, use
cargo install --path wikitrans --force
```

## Help
```
$ wikitrans --help

wikitrans 0.1.2
timvisee <timvisee@gmail.com>
Translate terms using Wikipedia

USAGE:
    wikitrans [OPTIONS] <TERM>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --language <language>      The search language tag [aliases: search]
    -t, --translate <translate>    The translate language tag

ARGS:
    <TERM>...    The term to search and translate
```

## License
This project is released under the GNU GPL-3.0 license.
Check out the [LICENSE](LICENSE) file for more information. 

[cargo]: https://github.com/rust-lang/cargo
[git]: https://git-scm.com/
[rustup]: https://rustup.rs/
[skim]: https://github.com/lotabout/skim
[usage-demo-asciinema]: https://asciinema.org/a/201904
[usage-demo-svg]: https://cdn.rawgit.com/timvisee/wikitrans/57a356be/res/demo.svg
