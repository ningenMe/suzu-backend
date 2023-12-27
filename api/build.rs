fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("./proto/suzu/v1/suzu.proto")?;
    Ok(())
}
