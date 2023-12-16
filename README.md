# cpasswordgenerator

A simple C program for generating random passwords, which creates strong
password(s) with lowercase letters, uppercase letters, and numbers.

## Table of Contents

1.  [Installation](#installation)
2.  [Usage](#usage)
3.  [Contributing](#contributing)
4.  [License](#license)
5.  [Contact](#contact)

## Installation

Before installing, [glibc](https://en.wikipedia.org/wiki/Glibc) is needed to
use this library, as it uses the `argp.h` library for command-line argument handling.
To compile and run this program, follow these steps:

1.  Clone this repository:
    ``` console
    git clone https://github.io/walker84837/cpasswordgenerator.git
    ```
2.  Open the terminal and navigate to the directory containing the source code.
3.  Compile the code using `make`:
    ``` console
    make
    ```

## Usage

The program accepts an optional command-line argument to specify the number
of passwords to generate. If no argument is provided, it defaults to generating one password.
There's nothing much to it.

1.  Compile the code as described in the [Installation](#installation) section.
2.  Run the program with an optional argument to specify the number of passwords.
    For instance, to generate 10 passwords:
    ``` console
    ./cpasswordgenerator -n 10
    ```

## Contributing

Contributions are welcome\! If you'd like to contribute, please:

  - Follow the [Linux kernel coding style](https://docs.kernel.org/process/coding-style.html).
      - Make sure the code can be compiled, using the `-std=c89` flag.
  - Follow the [code of conduct](CODE_OF_CONDUCT.md).
  - If you have to use an external header, please use lightweight ones.
  - Prefer using the standard library, or other headers, over reinventing the wheel.
  - For big changes (let's take a new feature), open an issue, and describe the
    following points:
      - Why should it be added? What does it add and why should it even be considered?
      - What's the difference between using it and not using it?
      - Keep in mind that this project is supposed to be simple.

## License

This project is licensed under the GNU General Public License, version 3. See
the [license](LICENSE.md) file for details or the [website](https://www.gnu.org/licenses/gpl-3.0.html).

## Contact

If you have any questions or need further assistance, you can contact me at
<walker84837@gmail.com>. **BEFORE** asking questions, please [ask for help in the smart way](http://catb.org/~esr/faqs/smart-questions.html).
