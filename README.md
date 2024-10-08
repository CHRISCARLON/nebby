# Nebby 🤥

Nebby is a command-line tool designed to quickly review basic information about a range of different file formats.

It provides various commands to interact with Excel, JSON, and CSV files - with more to come!

## Features

- **Excel File Handling**: Display basic information, check formatting, and quick view essential info.
- **CSV Support**: Basic CSV sniff feature.
- **JSON Handling**: Experimental basic API request feature - see how nested an json response is.
- **File Byte Inspection**: Examine the bytes of any file - useful for getting info on file signatures.
- **Flexible Command Interface**: Easily extendable with new commands.

## Installation

To install Nebby, use the provided `nebbybuild` script. This script will build and install the binary for you:

1. Clone the repository:
   ```bash
   git clone git@github.com:enmeshed-analytics/duckdb-postgis.git
   cd nebby
   ```

2. Give execute permission to the build script:
   ```bash
   chmod +x nebbybuild
   ```

3. Run the build script:
   ```bash
   ./nebbybuild
   ```

This script will build the project in release mode and install the `nebb` binary to `/usr/local/bin/`, making it accessible from anywhere on your system.

## Usage

Run Nebby from the command line to access its features:

```bash
nebb <COMMAND> <URL>
```

### Commands

- `basic-xl`: Display basic information about an Excel file.
- `format-xl`: Check formatting of an Excel file.
- `quick-view-xl`: Quick view of an Excel file.
- `basic-idx-xl`: Experimental feature to display basic information with a specified header index.
- `basic-json`: Experimental basic API request feature.
- `nibble`: Check bytes of any file.
- `basic-csv`: Basic CSV feature.
- `help`: Print help message or the help of the given subcommand(s).

### Options

- `-h`, `--help`: Print help information.
- `-V`, `--version`: Print version information.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License.
