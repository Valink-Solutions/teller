use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
};

use base64::{engine::general_purpose, Engine as _};

pub fn encode_image_to_base64(path: PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = fs::File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let res_base64 = general_purpose::STANDARD_NO_PAD.encode(&buf);
    Ok(format!("data:image/png;base64,{}", res_base64))
}

pub fn calculate_dir_size<P: AsRef<Path>>(path: P) -> std::io::Result<u64> {
    let mut size = 0;

    for entry in fs::read_dir(path)? {
        let dir = entry?;
        let metadata = dir.metadata()?;

        if metadata.is_dir() {
            size += calculate_dir_size(dir.path())?;
        } else {
            size += metadata.len();
        }
    }

    Ok(size)
}
