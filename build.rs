fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut prost_config = prost_build::Config::new();
    prost_config.protoc_arg("--experimental-allow-proto3-optional");

    tonic_build::compile_protos("proto/helloworld.proto")?;
    tonic_build::compile_protos("proto/calculator.proto")?;
    // tonic_build::configure()
    //     .build_server(false)
    //     .compile_with_config(prost_config, &["proto/contact.proto"], &["proto"])?;
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(&["proto/contact.proto"], &["proto"])?;

    // tonic_build::compile_protos("proto/contact.proto")?;
    Ok(())
}
