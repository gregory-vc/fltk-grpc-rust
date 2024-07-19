fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("src/proto/hello.proto")?;
    tonic_build::compile_protos("src/proto/solar-system-info.proto")?;
    Ok(())
}