// use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        // .file_descriptor_set_path(out_dir.join("api_descriptor.bin"))
        // .out_dir("./src")
        .compile(&["protos/cache.proto", "protos/cluster.proto"], &["protos"])?;
    Ok(())
}
