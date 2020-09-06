use autorust_openapi::v2::Spec;
use std::{fs::File, io::Read};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn assert_deserialize_without_ignored(paths: Vec<&str>) -> Result<()> {
    for path in paths {
        println!("  test deserialize {:?}", path);
        let mut bytes = Vec::new();
        File::open(path)?.read_to_end(&mut bytes)?;
        let deserializer = &mut serde_json::Deserializer::from_slice(&bytes);
        let mut ignored = Vec::new();
        let _spec: Spec = serde_ignored::deserialize(deserializer, |path| {
            let path = path.to_string();
            println!("    ignored {}", &path);
            ignored.push(path);
        })?;
        assert_eq!(0, ignored.len());
    }
    Ok(())
}