# Danalyze

Danalyze is a command-line tool to analyze and display directory contents with sorting and filtering options. This tool
lists files in a directory and provides various options to sort the files by size, name, created date, modified date, or
extension.

## Features

- List files in a specified directory
- Sort files by size, name, created date, modified date, or extension
- Display file sizes in a human-readable format (bytes, KB, MB, etc.)
- Print total size of all files in the directory

## Installation

### Using Cargo

You can install Danalyze using Cargo, the Rust package manager. First, ensure you have Rust and Cargo installed. Then,
run:

```sh
cargo install danalyze
```

### Using binary sources

Go to the [Releases](https://github.com/phibersoft/danalyze/releases) page and download the binary for your platform.
Extract the archive and run the binary from the command line.

## Usage

After installation, you can run the application from the command line:

```sh
danalyze [OPTIONS]
```

### Options

- `-p, --path <PATH>`: Path of the directory to analyze (default: `./`)
- `-o, --ordering <ORDERING>`: Order by (size, name, created_date, modified_date, extension) (default: `size`)
- `-d, --order-direction <ORDER_DIRECTION>`: Order direction (asc, desc) (default: `asc`)

### Examples

List files in the current directory, sorted by size in descending order:

```sh
danalyze --ordering size --order-direction desc
```

List files in a specific directory, sorted by name in ascending order:

```sh
danalyze --path /path/to/directory --ordering name --order-direction asc
```

## Output

The application will output a table with the following columns:

- `Name`: The name of the file
- `Size`: The human-readable size of the file
- `Created Date`: The creation date of the file
- `Modified Date`: The last modified date of the file
- `Extension`: The file extension

Example output:

```
Path: /path/to/directory
Ordering: size (desc)
Total size: 12.34 MB

+--------------------+----------+---------------------+---------------------+-----------+
| Name               | Size     | Created Date        | Modified Date       | Extension |
+--------------------+----------+---------------------+---------------------+-----------+
| file1.txt          | 1.23 MB  | 01/01/2020 12:00:00 | 01/01/2020 12:00:00 | txt       |
| file2.jpg          | 5.67 MB  | 01/01/2020 12:00:00 | 01/01/2020 12:00:00 | jpg       |
| file3.zip          | 5.44 MB  | 01/01/2020 12:00:00 | 01/01/2020 12:00:00 | zip       |
+--------------------+----------+---------------------+---------------------+-----------+
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any bugs, features, or improvements.

## Acknowledgements

- [chrono](https://crates.io/crates/chrono) for date and time handling
- [fs_extra](https://crates.io/crates/fs_extra) for file system operations
- [tabled](https://crates.io/crates/tabled) for table formatting

## TODO
- [ ] Add tests
- [ ] Add linux and macos binaries