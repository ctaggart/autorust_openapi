use autorust_openapi::OpenAPI;
use std::{fs::File, io::Read, process::exit};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// cargo run --example prettyprint -- ../azure-rest-api-specs/specification/security/resource-manager/common/v1/types.json
fn main() -> Result<()> {
    match std::env::args().nth(1) {
        None => {
            eprintln!("Please pass in the spec path.");
            exit(1);
        }
        Some(path) => {
            // reading the whole file upfront is much faster than using a BufReader
            // https://github.com/serde-rs/json/issues/160
            let mut bytes = Vec::new();
            File::open(path)?.read_to_end(&mut bytes)?;
            let spec: OpenAPI = serde_json::from_slice(&bytes)?;

            println!("{}", serde_json::to_string_pretty(&spec)?);
        }
    }
    Ok(())
}
