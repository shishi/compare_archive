use anyhow::Result;

#[derive(Debug)]
struct ZipFile {}

async fn read_zip_file() -> Result<ZipFile> {
    let mut zip = zip::ZipArchive::new(std::fs::File::open("foo.zip")?)?;
    return Ok(ZipFile {});
}
