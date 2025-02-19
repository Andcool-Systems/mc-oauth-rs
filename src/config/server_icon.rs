use base64_light::base64_encode_bytes;
use tokio::fs;

pub async fn load(file_path: &str) -> anyhow::Result<String> {
    let file_content = fs::read(file_path).await?;
    Ok(base64_encode_bytes(&file_content))
}
