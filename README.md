# Recast

Transforms one serialization format into another. Available serialization
formats include:

- JSON (`application/json`)
- TOML (`application/toml`)
- Yaml (`text/x-yaml`)
- [Query strings](https://github.com/ljharb/qs)
- CSV (`text/csv`)
- XML (`text/xml`)

## Installation

| Package manager | Command                |
| --------------- | ---------------------- |
| Cargo           | `cargo install recast` |

The [releases page](https://github.com/Altair-Bueno/recast/releases) contains
pre-compiled releases and shell completion files

## Usage

```text
Transforms one serialization format into another

Usage: recast [OPTIONS] [FILE]

Arguments:
  [FILE]  Input from file

Options:
  -f, --from <FORMAT>  Input format [default: json] [possible values: json, toml, yaml, query, csv, xml]
  -t, --to <FORMAT>    Output format [default: json] [possible values: json, toml, yaml, query, csv, xml]
  -o, --out <FILE>     Output to file
  -h, --help           Print help information (use `--help` for more detail)
  -V, --version        Print version information
```

## Examples

## [jq](https://stedolan.github.io/jq/)

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

## Syntax highlighting

`recast` doesn't provide syntax highlighting, but you can leverage other cli
tools like [bat](https://github.com/sharkdp/bat) for this purpose

```sh
recast -f toml Cargo.toml | bat --language json
```
