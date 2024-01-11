# Rust Cat Command

`rcat` is a simple Rust implementation of the classic Unix `cat` command. It concatenates and displays the content of files, providing options to number lines, display line endings, and suppress repeated empty lines.

## Installation
To use `rcat`, make sure you have Rust installed. Then, you can install it using Cargo:

```bash
git clone git@github.com:renatodinizc/rcat.git
cd rcat
cargo build --release
```

The executable will be available in the `target/release` directory. You can either run it directly from there or copy it to a directory in your system's `PATH`.

## Usage
```bash
rcat [OPTIONS] [FILES...]
```

## Command-Line Options
- `-n, --number`: Number all output lines.
- `-b, --number-nonblank`: Number nonempty output lines, overrides `-n`.
- `-E, --show-ends`: Display `$` at the end of each line.
- `-s, --squeeze-blank`: Suppress repeated empty output lines.

## Examples
```bash
# Display content of a file with line numbers
rcat -n file.txt

# Display content of multiple files with line numbers and line endings
rcat -n -E file1.txt file2.txt

# Display content from stdin
echo "Hello, World!" | rcat
```

## Contributing
Feel free to contribute to the project by opening issues or creating pull requests on the [GitHub repository](https://github.com/yourusername/rcat).

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.