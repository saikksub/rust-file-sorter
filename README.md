# Rust File Organizer

## Overview
The Rust File Organizer is a command-line tool written in Rust for organizing and sorting files in a directory. It allows users to sort files based on various criteria such as file type, size, name, or date. This tool is designed to be fast, efficient, and safe, preventing accidental data loss.

## Features
- **Sorting Criteria**: Organize files by type, size, name, or date.
- **User Interaction**: Easy-to-use command-line arguments or interactive mode.
- **Safety**: Prevents data loss with confirmation prompts and safe file operations.
- **Logging**: Summary of actions and detailed logs.
- **Dry Run Mode**: Preview changes without altering any files.
- **Configurable Settings**: Custom rules for sorting and saving settings for future use.

## Getting Started

### Prerequisites
- Rust and Cargo installed on your system.

### Installation
1. Clone the repository:
2. Change directory:
``` bash
cd rust-file-organizer
```
3. Build the project:
``` bash
cargo build --release
```
### Usage
Run the program with the required arguments:

Options:
- `-t, --type`: Sort by file type.
- `-s, --size`: Sort by file size.
- `-n, --name`: Sort by file name.
- `-d, --date`: Sort by date.
- `--dry-run`: Preview changes without executing.

For more detailed information, use the help command:
``` bash
cargo run -- --help
```

## License
This project is licensed under [MIT License](LICENSE).









