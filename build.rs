

fn main() {
    tonic_build::configure()
        .compile_protos(&["wad-proto/echo.proto"], &["proto"])
        .unwrap();
}