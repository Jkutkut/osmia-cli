pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const HELP: &str =
"--code          -- Add code as a JSON file                  
--code-in       -- Add code as a JSON string from stdin     
--code-str      -- Add code as a JSON string                
--ctx           -- Add context as a JSON file               
--ctx-in        -- Add context as a JSON string from stdin  
--ctx-str       -- Add context as a JSON string             
--help      -h  -- Display help information                 
--version   -v  -- Display current version";

pub const BIN_NAME: &str = env!("CARGO_BIN_NAME");
