use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncReadExt;

async fn get_file(url: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = File::open(url).await?;
    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;
    Ok(contents)
}

async fn get_file_string(url: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(String::from(std::str::from_utf8(&get_file(url).await?)?))
}

pub async fn get_static(url: &str) -> Option<String> {
    get_file_string(format!("/var/static/root/{}", url)).await.ok()
}
