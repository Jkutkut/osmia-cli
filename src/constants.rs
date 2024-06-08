pub const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

pub const HELP: &str =
"--code          -- Add code as a JSON file                  
--code-in       -- Add code as a JSON string from stdin     
--code-str      -- Add code as a JSON string                
--ctx           -- Add context as a JSON file               
--ctx-in        -- Add context as a JSON string from stdin  
--ctx-str       -- Add context as a JSON string             
--help      -h  -- Display help information                 
--version   -v  -- Display current version";


