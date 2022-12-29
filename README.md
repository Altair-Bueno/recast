# Recast

Transforms one serialization format into another. Available serialization
formats include:

- JSON (`application/json`)
- TOML (`application/toml`)
- Yaml (`text/x-yaml`)
- [Query strings](https://github.com/ljharb/qs)
- CSV (`text/csv`)

## Installation

| Package manager | Command                |
| --------------- | ---------------------- |
| Cargo           | `cargo install recast` |

## Usage

```text
Transforms one serialization format into another

Usage: recast [OPTIONS] [FILE]

Arguments:
  [FILE]  File to transform

Options:
  -f, --from <FROM>  Input format [default: json] [possible values: json, toml, yaml, query, csv]
  -t, --to <TO>      Output format [default: json] [possible values: json, toml, yaml, query, csv]
  -o, --out <OUT>    Output file
  -h, --help         Print help information (use `--help` for more detail)
  -V, --version      Print version information
```

## Examples

Leverage jq's powerful filters on other serialization formats

```sh
$ recast -f toml Cargo.lock \
  | jq '.package | map({name: .name, version: .version}) | sort_by(.name)' \
  | recast -t csv \
  | head -n 5
name,version
addr2line,0.19.0
adler,1.0.2
autocfg,1.1.0
backtrace,0.3.67
```
