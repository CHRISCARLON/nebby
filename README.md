# Nebby 🤥

Nebby is a command-line tool for quickly reviewing basic information about remote Excel (xlsx) files and making simple API GET requests.

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
- [ ] Enhance JSON processing capabilities
- [ ] Implement DataFrame support
- [ ] TBC

## Author

Christopher Carlon

## Version

0.1.3
