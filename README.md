# cpasswordgenerator

This is a simple C program for generating random passwords. It creates strong and unique passwords by combining lowercase letters, uppercase letters, and numbers.

## Table of Contents

1.  [Installation](#installation)
2.  [Usage](#usage)
3.  [Contributing](#contributing)
4.  [License](#license)
5.  [Contact](#contact)

## Installation

To compile and run this program, follow these steps:

1.  Ensure you have a C compiler, such as GCC, installed on your system.

2.  Clone this repository: `git clone https://github.io/walker84837/cpasswordgenerator.git`

3.  Open the terminal and navigate to the directory containing the source code.

4.  Compile the code using `make`:

    ``` bash
    make
    ```

## Usage

The program accepts an optional command-line argument to specify the number of passwords to generate. If no argument is provided, it defaults to generating one password.

1.  Compile the code as described in the Installation section.

2.  Run the program with an optional argument to specify the number of passwords. For instance, to generate 10 passwords:

    ``` bash
    ./cpasswordgenerator -n 10
    ```

**Warning**: The program clamps the number of passwords to a range of 1 to 2048. If you exceed this limit, it will be automatically adjusted to 2048.

## Contributing

If you'd like to contribute to this project, please follow these guidelines:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix: `git switch -c branchname`.
3.  Make your changes and commit them.
4.  Push your changes to your fork: `git push origin branchname`.
5.  Create a pull request to the `main` branch of the original repository.

## License

This project is licensed under the GNU GPLv3 License. See the [LICENSE](LICENSE.md) file for details.

## Contact

If you have any questions or need further assistance, you can contact me at <walker84837@gmail.com>.

