use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::io::prelude::*;

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
    let url = format!("/var/static/root/{}", url);
    println!("Searching for file with url {}", &url);
    Some(get_file_string(url.clone())
        .await
        .expect(&format!("Failed to open file {}", url)))
}

pub async fn get_image(username: &str, gallery: &str, image_title: &str) -> Option<Vec<u8>> {
    let url = format!("/var/static/root/u/{}/{}/{}", username, gallery, image_title);
    println!("Searching for image with url {}", &url);
    Some(get_file(url.clone())
        .await
        .expect(&format!("Failed to open file {}", url)))
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
        let split_gallery_display = vec![split_template[2], &gallery_url, split_template[3], &gallery_name, split_template[4]];
        let gallery_display: String = split_gallery_display.into_iter().collect();
        gallery_displays.push(gallery_display);
    }
    for i in 0..gallery_displays.len() {
        split_file.push(&gallery_displays[i]);
    }
    split_file.push(split_template[5]);
    split_file.into_iter().collect()
}

pub async fn get_gallery_page(username: &str, gallery: &str, images: Vec<String>) -> String {
    let user_template = get_file_string(String::from("/var/static/gallery.html")).await.expect("Failed to find gallery.html");
    let split_template: Vec<&str> = user_template.split('$').collect();
    let mut split_file = vec![split_template[0], username, split_template[1], gallery, split_template[2]];
    let mut image_displays: Vec<String> = vec![];
    for image in images {
        let image_url = format!("/u/{}/{}/{}", username, gallery, image);
        let split_image_display = vec![split_template[3], &image, split_template[4], &image_url, split_template[5]];
        image_displays.push(split_image_display.into_iter().collect());
    }
    for i in 0..image_displays.len() {
        split_file.push(&image_displays[i]);
    }
    split_file.push(split_template[6]);
    split_file.into_iter().collect()
}

pub fn make_user_dir(username: String) {
    std::fs::create_dir_all(format!("/var/static/root/u/{}", username)).expect("Failed to create user dir");
}

pub fn make_gallery_dir(username: String, galleryname: String) {
    std::fs::create_dir_all(format!("/var/static/root/u/{}/{}", username, galleryname)).expect("Failed to create gallery dir");
}

pub fn make_image(username: String, galleryname: String, imagetitle: String, image: String) {
    let mut image_file = std::fs::File::create(format!("/var/static/root/u/{}/{}/{}", username, galleryname, imagetitle)).expect("Failed to create image file");
    image_file.write_all(&base64::decode(image.split("image/jpeg;base64,").skip(1).next().expect("Bad format for image")).expect("Failed to decode image")).expect("Failed to save image file");
}
