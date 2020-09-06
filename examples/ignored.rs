use autorust_openapi::Spec;
use std::{fs::File, io::Read, process::exit};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// cargo run --example ignored -- data/v2/k8s.json
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
            // instead of reading directly from the bytes, wrap the deserializer with serde_ignored
            // let spec: Spec = serde_json::from_slice(&bytes)?;
            let deserializer = &mut serde_json::Deserializer::from_slice(&bytes);
            let _spec: Spec = serde_ignored::deserialize(deserializer, |path| {
                println!("ignored {}", path.to_string());
            })?;
        }
    }
    Ok(())
}
