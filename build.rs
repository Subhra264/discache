// use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(&["protos/cache.proto", "protos/cluster.proto"], &["protos"])?;
    Ok(())
}
