fn main() {
    tonic_build::configure()
        .build_server(false)
        .compile_protos(
            &["proto/message.proto"],
             &["proto"],)
        .unwrap();
    tauri_build::build()
}
