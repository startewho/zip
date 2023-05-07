use std::io;

pub fn main() -> io::Result<()> {
    let path = std::env::args().nth(1).expect("Usage: zip-extract <path>");
    let dir = tempdir::TempDir::new("zip-extract")?;
    println!("{dir:?}");
    let mut decompressor = Default::default();
    for file in &zip::Archive::open_at(path)?.try_map_disks(|f| file_content::FileContent::from_file(&f))? {
        // TODO: Rework the API to allow
        //   A) extractor.bufread(file)?.copy_to(stdout);
        //   B) file.extract_to(stdout)?;
        //   C) file.bufread()?.copy_to(stdout); // note that this requires an owned Read<'extractor>
        let mut out = std::fs::File::create(dir.path().join(core::str::from_utf8(file.name()).unwrap()))?;
        let mut reader = file
            .map_disk(file_content::FileCursor::new)
            .reader()?
            .remove_encryption_io()?
            .or_else(|d| d.try_password(b"password"))?
            .build_with_buffering(&mut decompressor, std::io::BufReader::new);
        // io::copy(&mut reader, &mut std::io::stdout())?;
        std::io::copy(&mut reader, &mut out)?;
    }

    if cfg!(debug_assertions) {
        core::mem::forget(dir);
    }
    Ok(())
}