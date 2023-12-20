// Acts as a build script for tonic-build
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This is compiling the payments.proto file from the proto directory using tonic-build
    tonic_build::compile_protos("proto/payments.proto")?;
    Ok(())
}