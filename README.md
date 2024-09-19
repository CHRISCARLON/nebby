# Nebby 🤥

Nebby is a command-line tool for all your nosey parkers out there.

Quickly review basic information about a range of different file formats.

> [!NOTE]
> Nebby is currently under active development. Features and commands may change as the project evolves.

## Usage

```
nebb <COMMAND>
```

## Commands

### `basic`
Display basic information about an Excel file.
```
nebb basic <URL>
```

### `format`
Check formatting of an Excel file.
```
nebb format <URL>
```

### `quick-view`
Quick view of an Excel file.
```
nebb quick-view <URL>
```

### `basic-idx`
Display basic information about an Excel file with a specified header index.
This is useful if the header is not on the first row.
```
nebb basic-idx <URL> [--header-index <INDEX>]
```

### `basic-json`
Make a basic API GET request and display information about how nested the JSON response is.
```
nebb basic-json <URL>
```

## Options

- `-h`, `--help`: Print help information
- `-V`, `--version`: Print version information

## Roadmap

- [ ] Add support for additional file formats
- ✅ Enhance JSON processing capabilities
- [ ] TBC

## Author

Christopher Carlon

## Version

0.1.3
