# pwdgen

A simple Rust program for generating random passwords, which creates strong
password(s) with lowercase letters, uppercase letters, and numbers.

## Table of Contents

1.  [Installation](#installation)
2.  [Usage](#usage)
3.  [Contributing](#contributing)
4.  [License](#license)
5.  [Contact](#contact)

## Installation

To build this repository from source, you need Cargo and Rust. You can download
Rust at <https://rustup.rs>. Here is how you build from source:

1.  Clone this repository:
    ``` console
    git clone https://github.io/walker84837/pwdgen-rs.git
    cd pwdgen-rs
    ```
2.  Compile the code using `cargo`:
    ``` console
    cargo b -r
    ```

## Usage

The program accepts an optional command-line argument to specify the number of
passwords to generate. If no argument is provided, it defaults to generating one
password. There's nothing much to it.

Run the program with an optional argument to specify the number of passwords.
For instance, to generate 10 passwords:

``` console
./pwdgen -n 10
```

## Contributing

Contributions are welcome! If you'd like to contribute, please:

  - Follow the [code of conduct](CODE_OF_CONDUCT.md).
  - If you have to use an external header, please use lightweight ones.
  - Prefer using the standard library, or other headers, over reinventing the
    wheel.
  - When proposing changes, open an issue.

## License

This project is licensed under the GNU General Public License, version 3. See
the [license](LICENSE.md) file for details or the
[website](https://www.gnu.org/licenses/gpl-3.0.html).

## Contact

If you have any questions or need further assistance, you can contact me at
<walker84837@gmail.com>. **BEFORE** asking questions, please [ask for help in
the smart way](http://catb.org/~esr/faqs/smart-questions.html).
