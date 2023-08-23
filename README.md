# Rust Note Taking Tool

This is a simple command-line note-taking tool written in Rust. It allows 
you to create and store notes in files with timestamps.

## Prerequisites

Before using this tool, ensure you have the following installed on your 
system:

- Rust: You can download and install Rust from 
[rust-lang.org](https://www.rust-lang.org/tools/install).

## Installation

To use this tool, you can clone the repository and build the Rust binary:

```shell
git clone https://github.com/your/repo.git
cd repo
cargo build --release
```

## Usage

You can use this tool to create and store notes. Here are the available 
commands and options:

```shell
notes [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --filename <filename>    Sets the filename for the note
    -n, --note <note>            Sets the content of the note

```

### Example Usage

- To create a new note with a specific filename:
  ```shell
  notes -f my_note -n "This is my note content."
  ```

- If you don't specify a filename or note content, the tool will prompt 
you to enter them interactively.

- Notes are stored in the `/Users/<username>/Notes/` directory with 
filenames in the format `filename_<timestamp>.note`.

## Author

- Author: Huub Verbeek

## License

This project is licensed under the MIT License - see the 
[LICENSE](LICENSE) file for details.

## Acknowledgments

- This tool was created as a simple example of Rust command-line 
application development.

Feel free to modify and use this tool according to your needs. Enjoy 
note-taking in Rust!
