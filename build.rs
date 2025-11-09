use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let build_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    #[cfg(feature = "server")]
    tonic_prost_build::configure()
        .file_descriptor_set_path(build_path.join("route-forecast_binary.bin"))
        .compile_protos(&["proto/route-forecast.proto"], &["proto"])?;

    #[cfg(feature = "wasm")]
    tonic_prost_build::configure()
        .build_transport(false)
        .file_descriptor_set_path(build_path.join("route-forecast_binary.bin"))
        .compile_protos(&["proto/route-forecast.proto"], &["proto"])?;

    Ok(())
}
