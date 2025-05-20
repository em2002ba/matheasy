# Matheasy


Matheasy is a simple command-line calculator written in Rust. It evaluates mathematical expressions from command line input or from a file and displays the results.


## Features



- Evaluate arithmetic expressions with operators like `+`, `-`, `*`, `/`, and parentheses.
- Run calculations directly via command line arguments.
- Process multiple expressions from a text file.

## Usage


### Run a single expression

```bash
cargo run -- "3 + 4 * 2"
Output:


3 + 4 * 2 = 11
Run expressions from a file
Create a file named math.txt with expressions, for example:


3 + 4 * 2
(8 - 3) / 2
5 * 5 + 1
7 + (6 / 2) * 3
Run the program with:

bash

cargo run -- --file math.txt

Output:


3 + 4 * 2 = 11
(8 - 3) / 2 = 2.5
5 * 5 + 1 = 26
7 + (6 / 2) * 3 = 16
Build
Make sure you have Rust installed. Then build the project:

bash
cargo build
License
MIT License.

Created by [babirye nambuusi emily]
