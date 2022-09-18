use anyhow::Result;

#[derive(Debug)]
pub struct ZipFile {}

pub async fn read_zip_file() -> Result<ZipFile> {
    // it will receive file path as args and loop
    let mut zip = zip::ZipArchive::new(std::fs::File::open("foo.zip")?)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        println!(
            "File {} has size {} and is compressed to {} bytes and CRC32 is {}",
            file.name(),
            file.size(),
            file.compressed_size(),
            format!("{:x}", file.crc32())
        );
    }
    return Ok(ZipFile {});
}
