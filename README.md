# CryptWeave

A command-line utility for generating passwords, containing strong lowercase
letters, uppercase letters, and numbers.

## Table of Contents

  - [Installation](#installation)
  - [Usage](#usage)
  - [Contributing](#contributing)
  - [License](#license)

## Installation

To build this repository from source, you need Cargo and Rust. You can download
Rust at <https://rustup.rs>. Here is how you build from source:

1.  Clone this repository:
    ``` console
    $ git clone https://github.com/walker84837/cryptweave-rs.git
    $ cd cryptweave-rs
    ```
2.  Compile and run the code:
    ``` console
    $ cargo r -r -- [ARGS]
    ```

### Arch Linux

There is a PKGBUILD in the [archlinux](archlinux/) folder.

## Usage

If no argument is provided, the program defaults to generating one password
with 3 [sections](docs/sections.md), unless specified otherwise. For instance,
to generate 10 passwords with 2 sections:

``` console
$ cwv -n 10 --sections 2 
```

## Contributing

Contributions are welcome! If you'd like to contribute, please:

  - Follow the [code of conduct](CODE_OF_CONDUCT.md).
  - If you have to use an external crate, please use lightweight ones if
    possible.
  - Prefer using the standard library, or other headers, over reinventing the
    wheel.
  - Open an issue when proposing changes.

If you have any questions or need further assistance, you can contact me at
`@winlogon.exe:matrix.org`. **BEFORE** asking questions, please [ask for help
the smart way](http://catb.org/~esr/faqs/smart-questions.html).

## License

This project is licensed under the GNU General Public License, version 3. See
the [license](LICENSE.md). You can get a copy at
[website](https://www.gnu.org/licenses/gpl-3.0.html).
