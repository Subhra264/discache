use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let project_dir = {
        let mut this_dir = PathBuf::from(file!());
        this_dir.pop();
        this_dir
    };

    env::set_var(
        "CFLAGS",
        format!(
            "-I{dir}/external_lib/xxhash -DXXH_STATIC_LINKING_ONLY=1 -DXXH_NO_STREAM=1 {old}",
            dir = project_dir.display(),
            old = env::var("OUT_DIR").unwrap_or("".to_string())
        ),
    );

    cc::Build::new()
        .file("external_lib/xxHash/xxhash.c")
        .compile("xxhash");

    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .clang_arg("-I./external_lib/xxHash/")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()?;

    bindings.write_to_file(out_dir.join("bindings.rs"))?;

    tonic_build::configure()
        .compile(&["protos/cache.proto", "protos/cluster.proto"], &["protos"])?;
    Ok(())
}
