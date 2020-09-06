pub mod v2;

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::{self}, io::Read,
    };
    use fs::File;
    use v2::Spec;

    pub type Error = Box<dyn std::error::Error>;
    pub type Result<T> = std::result::Result<T, Error>;

    // Just tests if the deserialization does not blow up. But does not test correctness
    #[test]
    fn can_deserialize() -> Result<()> {
        for entry in fs::read_dir("data/v2")? {
            let path = entry?.path();
            // cargo test -- --nocapture to see this message
            println!("  test deserialize {:?}", path);
            let mut bytes = Vec::new();
            File::open(path)?.read_to_end(&mut bytes)?;
            let _spec: Spec = serde_json::from_slice(&bytes)?;
        }
        Ok(())
    }

}
