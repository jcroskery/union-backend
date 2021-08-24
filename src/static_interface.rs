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
    get_file_string(format!("/var/static/root/{}", url))
        .await
        .ok()
}

pub async fn get_user_page(username: &str, gallery_names: Vec<String>) -> String {
    let user_template = get_file_string(String::from("/var/static/users.html"))
        .await
        .expect("Failed to find users.html");
    let split_template: Vec<&str> = user_template.split('$').collect();
    let mut split_file = vec![split_template[0], username, split_template[1]];
    let mut gallery_displays = vec![];
    for gallery_name in gallery_names {
        let gallery_url = format!("/u/{}/{}", username, gallery_name);
        let split_gallery_display = vec![split_template[2], &gallery_name, split_template[3], &gallery_url, split_template[4]];
        let gallery_display: String = split_gallery_display.into_iter().collect();
        gallery_displays.push(gallery_display);
    }
    for i in 0..gallery_displays.len() {
        split_file.push(&gallery_displays[i]);
    }
    split_file.push(split_template[5]);
    split_file.into_iter().collect()
}
