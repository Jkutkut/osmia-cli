# Osmia CLI:
A simple command line tool to run [osmia](https://github.com/jkutkut/osmia).
It features a ZSH completion script and multiple arguments for code and context.

## Usage
```bash
--code          -- Add code as an osmia file
--code-in       -- Add code as an osmia string from stdin
--code-str      -- Add code as an osmia string
--ctx           -- Add context as a JSON file
--ctx-in        -- Add context as a JSON string from stdin
--ctx-str       -- Add context as a JSON string
--help      -h  -- Display help information
--version   -v  -- Display current version
```

## Examples
### Hello World
```bash
osmia --ctx-str '{"usr": {"name": "Marvin"}}' --code-str "Hello {{usr.name}}"
```
```text
Hello Marvin
```

### Context from file:
```bash
osmia --ctx ./context.json --code-str "Hello {{usr.name}}"
```
```text
Hello Marvin
```

### Context from stdin:
```bash
echo '{"usr": {"name": "Marvin"}}' | osmia --ctx-in --code-str "Hello {{usr.name}}"
```
```text
Hello Marvin
```

### Process API response:
```bash
curl https://pokeapi.co/api/v2/pokemon/pikachu | \
    osmia --ctx-in --code-str \
    'The moves of {{ name }} are: {{ moves?map(fn (f) => f.move.name)?join(", ") }}.'
```

## Syntax:
This CLI uses the library [osmia](https://github.com/jkutkut/osmia) project. Your can have a look there or in their [documentation](https://jkutkut.github.io/osmia/osmia/struct.Osmia.html).

## Note:
This README asumes that you've renamed the binary `osmia-cli` to `osmia` and it's available in your PATH variable.
