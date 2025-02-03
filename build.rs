use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("file_descriptor.bin"))
        .compile_protos(
            &[
                "wad-proto/auth/v1/auth.proto",
                "wad-proto/dictionary/v1/dictionary.proto",
            ],
            &["proto"],
        )
        .unwrap();
}
