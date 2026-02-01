pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const HELP: &str =
"--code          -- Add code as an osmia file
--code-in       -- Add code as an osmia string from stdin
--code-str      -- Add code as an osmia string
--ctx           -- Add context as a JSON file
--ctx-json      -- Add context as a JSON file
--ctx-yaml      -- Add context as a YAML file
--ctx-in        -- Add context as a JSON string from stdin
--ctx-json-in   -- Add context as a JSON string from stdin
--ctx-yaml-in   -- Add context as a YAML string from stdin
--ctx-str       -- Add context as a JSON string
--ctx-json-str  -- Add context as a JSON string
--ctx-yaml-str  -- Add context as a YAML string
--help      -h  -- Display help information
--version   -v  -- Display current version";

pub const BIN_NAME: &str = env!("CARGO_BIN_NAME");
