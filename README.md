# Matheasy

**Matheasy** is a simple command-line calculator written in Rust. It evaluates mathematical expressions from the command line or from a file and displays the results clearly and efficiently.

## ✨ Features

- ✅ Evaluate arithmetic expressions with operators like `+`, `-`, `*`, `/`, and parentheses.
- 🧮 Run calculations directly via command-line arguments.
- 📂 Process multiple expressions from a text file.
- 🧱 Lightweight and fast – ideal for scripting, study, or day-to-day quick math.

## 📌 Use Case

This tool is particularly useful in the following contexts:

- **Business administration tasks** such as quick financial calculations, discounts, taxes, or forecasts without leaving the terminal.
- **Students** needing a fast way to validate homework answers or try out formulas.
- **Developers or Linux users** who want to integrate simple math into scripts or avoid switching to a GUI calculator.
- **Teaching or learning programming**, especially Rust, through a real-world utility.

By staying entirely in the terminal, **Matheasy** improves focus and boosts productivity, especially for command-line enthusiasts or when multitasking in shell environments.

## 🚀 Usage

### Run a single expression

```bash
cargo run -- "3 + 4 * 2"
Output:

3 + 4 * 2 = 11
Run expressions from a file
Create a file named math.txt with expressions, e.g.:

3 + 4 * 2
(8 - 3) / 2
5 * 5 + 1
7 + (6 / 2) * 3
Then run:

bash

cargo run -- --file math.txt
Output:

3 + 4 * 2 = 11
(8 - 3) / 2 = 2.5
5 * 5 + 1 = 26
7 + (6 / 2) * 3 = 16
🛠️ Build
Make sure you have Rust installed, then build the project:

cargo build
📦 Binary
A statically-linked Linux binary (aarch64) is included in the submission for direct use on Linux devices. Other builds (x86_64, Windows) are possible with cross or GitHub Actions.

📄 License
MIT License.

Created by Babirye Nambuusi Emily, Business Administration student.
