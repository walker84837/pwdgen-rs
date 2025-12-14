# cryptweave

A command-line utility for generating passwords, containing strong lowercase letters, uppercase letters, and numbers.

## Table of Contents

  - [Installation](#installation)
  - [Usage](#usage)
  - [Contributing](#contributing)
  - [License](#license)

## Project Overview

One day, while signing up on some website with my iPhone, I noticed that the format of the passwords was in this format:
```
XXXXXX-XXXXXX-XXXXXX
```
Where:
- the string is split into three sections (dashes)
- one of the characters is uppercase
- there is a number
- the rest are uppercase characters

I wanted the same generator on my PC, and I got inspired to make this, and this is where cryptweave comes from.

cryptweave generates passwords with:
- lowercase letters;
- uppercase letters;
- numbers.
making it strong, secure and easy to remember.

## Installation

To build this repository from source, you need Cargo and Rust. You can download
Rust at <https://rustup.rs>. Here is how you build from source:

1.  Clone this repository:
    ``` console
    $ git clone https://github.com/walker84837/cryptweave.git
    $ cd cryptweave
    ```
2.  Compile and run the code:
    ``` console
    $ cargo r -r -- [ARGS]
    ```
    or run the code at
    `./target/release/cryptweave`

### Arch Linux

There is a PKGBUILD in the [archlinux](archlinux/) folder.

## Usage

If no argument is provided, the program defaults to generating one password with 3 sections of 6 characters each, unless specified otherwise.

For instance, to generate 10 passwords with 2 sections and 8 characters per section:

``` console
$ cwv -n 10 -s 2 -l 8
```

## Contributing

Contributions are welcome! If you'd like to contribute, please:

  - Follow the [code of conduct](CODE_OF_CONDUCT.md) (Contributor Covenant).
  - Make sure to format your code with `cargo fmt` and lint it with `cargo clippy`
  - Open an issue when proposing changes

If you have any questions or need further help, you can contact me at `@winlogon.exe:matrix.org`.

## License

This project is licensed under the GNU General Public License, version 3. See the [license](LICENSE.md). You can get a copy at [website](https://www.gnu.org/licenses/gpl-3.0.html).
