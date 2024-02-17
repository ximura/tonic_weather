use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("weather_descriptor.bin"))
        .compile(&["./proto/weather.proto"], &["proto"])
        .unwrap();

    tonic_build::compile_protos("./proto/weather.proto")?;

    Ok(())
}
