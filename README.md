# cpasswordgenerator

This is a simple C program for generating random passwords, which creates strong passwords with lowercase letters, uppercase letters, and numbers.

## Table of Contents

1.  [Installation](#installation)
2.  [Usage](#usage)
3.  [License](#license)
4.  [Contact](#contact)

## Installation

Before installing, [`glibc`](link0) is needed to use this library, as it uses the `argp.h` library for command-line argument handling.
To compile and run this program, follow these steps:

2.  Clone this repository: `git clone https://github.io/walker84837/cpasswordgenerator.git`
3.  Open the terminal and navigate to the directory containing the source code.
4.  Compile the code using `make`:

    ``` bash
    make
    ```

## Usage

The program accepts an optional command-line argument to specify the number of passwords to generate. If no argument is provided, it defaults to generating one password. There's nothing much to it.

1.  Compile the code as described in the [Installation](#installation) section.
2.  Run the program with an optional argument to specify the number of passwords. For instance, to generate 10 passwords:

    ``` bash
    ./cpasswordgenerator -n 10
    ```

**Warning**: The program clamps the number of passwords to a range of 1 to 2048. This limit exists as if the user provided a number too high of passwords it will end up taking up a lot of the machine's RAM. 

## License

This project is licensed under the GNU GPL, version 3 license. See the [LICENSE](LICENSE.md) file for details or the [website](link1).

## Contact

If you have any questions or need further assistance, you can contact me at <walker84837@gmail.com>.

[link0]: https://en.wikipedia.org/wiki/Glibc "GNU C Library"
[link1]: https://www.gnu.org/licenses/gpl-3.0.html "GNU GPL, version 3"
