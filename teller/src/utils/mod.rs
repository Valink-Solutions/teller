use std::path::PathBuf;

use async_recursion::async_recursion;
use base64::{engine::general_purpose, Engine as _};
use tokio::{fs, io::AsyncReadExt};

pub async fn encode_image_to_base64(path: PathBuf) -> Result<String, std::io::Error> {
    let mut file = fs::File::open(path).await?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await?;
    let res_base64 = general_purpose::STANDARD_NO_PAD.encode(&buf);
    Ok(format!("data:image/png;base64,{}", res_base64))
}

#[async_recursion]
pub async fn calculate_dir_size(path: PathBuf) -> Result<u64, std::io::Error> {
    let mut size = 0;

    let mut main_dir = fs::read_dir(&path).await?;

    while let Some(dir) = main_dir.next_entry().await? {
        let metadata = dir.metadata().await?;

        if metadata.is_dir() {
            size += calculate_dir_size(dir.path()).await?;
        } else {
            size += metadata.len();
        }
    }

    Ok(size)
}
