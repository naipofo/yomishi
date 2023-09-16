use std::io::Result;

fn main() -> Result<()> {
    tonic_build::compile_protos("../proto/yomishi/scan.proto")?;
    tonic_build::compile_protos("../proto/yomishi/anki.proto")?;
    Ok(())
}
