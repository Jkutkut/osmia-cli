# Osmia CLI:
A simple, command line tool to use [osmia](https://github.com/jkutkut/osmia).

## Usage
```bash
osmia [--ctx [json-object]] [--help]
```

## Examples
```bash
echo "Hello {{name}}" | osmia --ctx '{"name": "Marvin"}'
```
```text
Hello Marvin
```
