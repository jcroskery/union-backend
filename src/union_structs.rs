use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;

const USERNAME_ERROR_MESSAGE: &str = "Usernames must be between 4 and 16 characters long with only letters, numbers, and underscores (_).";
const GALLERY_NAME_ERROR_MESSAGE: &str = "Gallery names must be between 1 and 128 characters long with only letters, numbers, and underscores (_).";
const IMAGE_TITLE_ERROR_MESSAGE: &str = "Image titles must be between 1 and 128 characters long with only letters, numbers, underscores (_), hyphens (-), hashtags (#), and periods (.). In addition, only image names that end with .jpg are valid.";
const PASSWORD_ERROR_MESSAGE: &str = "Passwords must be between 8 and 64 characters long.";
const LABEL_ERROR_MESSAGE: &str = "Labels must be between 4 and 64 characters long with only letters, numbers, underscores (_), and at signs (@). ";

lazy_static! {
    static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_]{4,16}$").unwrap();
    static ref GALLERY_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_]{1,128}$").unwrap();
    static ref IMAGETITLE_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_\-.#]{1,128}.jpg$").unwrap();
    static ref PASSWORD_REGEX: Regex = Regex::new(r"^.{8,64}$").unwrap();
    static ref LABEL_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_@]{4,64}$").unwrap();
    static ref EMAIL_REGEX: Regex =
        Regex::new(r"^(([a-z0-9_+.]{1,32})@([a-z0-9\-\.]{1,32})\.([a-z]{2,6}))$").unwrap();
    pub static ref ID_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9]{255}$").unwrap();
}

pub fn parse(regex: &Regex, unverified: &str) -> Option<String> {
    if regex.is_match(unverified) {
        Some(String::from(unverified))
    } else {
        None
    }
}
pub struct InputError {
    message: String,
}

impl InputError {
    pub fn new(message: Option<&str>) -> Self {
        InputError {
            message: String::from(message.unwrap_or("Unspecified Input Error")),
        }
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.message) // user-facing output
    }
}

impl fmt::Debug for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ error: {}, file: {}, line: {} }}",
            &self.message,
            file!(),
            line!()
        ) // programmer-facing output
    }
}
/*
pub struct Username {
    username: String,
}

impl Username {
    pub fn new(username: &str) -> Result<Self, InputError> {
        lazy_static! {
            static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_]{4,16}$").unwrap();
        }
        if USERNAME_REGEX.is_match(username) {
            Ok(Username {
                username: String::from(username)
            })
        } else {
            Err(InputError::new(Some(USERNAME_ERROR_MESSAGE)))
        }
    }
    pub fn get(&self) -> String {
        self.username.clone()
    }
}

pub struct Gallery {
    gallery: String,
}

impl Gallery {
    pub fn new(gallery: &str) -> Result<Self, InputError> {
        lazy_static! {
            static ref GALLERY_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_]{1,128}$").unwrap();
        }
        if GALLERY_REGEX.is_match(gallery) {
            Ok(Gallery {
                gallery: String::from(gallery)
            })
        } else {
            Err(InputError::new(Some(GALLERY_NAME_ERROR_MESSAGE)))
        }
    }
    pub fn get(&self) -> String {
        self.gallery.clone()
    }
}

pub struct ImageTitle {
    image_title: String,
}

impl ImageTitle {
    pub fn new(image_title: &str) -> Result<Self, InputError> {
        lazy_static! {
            static ref IMAGETITLE_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_\-.#]{1,128}.jpg$").unwrap();
        }
        if IMAGETITLE_REGEX.is_match(image_title) {
            Ok(ImageTitle {
                image_title: String::from(image_title)
            })
        } else {
            Err(InputError::new(Some(IMAGE_TITLE_ERROR_MESSAGE)))
        }
    }
    pub fn get(&self) -> String {
        self.image_title.clone()
    }
}


pub struct Password {
    password: String,
}

impl Password {
    pub fn new(password: &str) -> Result<Self, InputError> {
        lazy_static! {
            static ref PASSWORD_REGEX: Regex = Regex::new(r"^.{8,64}$").unwrap();
        }
        if PASSWORD_REGEX.is_match(password) {
            Ok(Password {
                password: String::from(password)
            })
        } else {
            Err(InputError::new(Some(PASSWORD_ERROR_MESSAGE)))
        }
    }
    pub fn get(&self) -> String {
        self.password.clone()
    }
}

pub struct Label {
    label: String,
}

impl Label {
    pub fn new(label: &str) -> Result<Self, InputError> {
        lazy_static! {
            static ref LABEL_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_@]{4,64}$").unwrap();
        }
        if LABEL_REGEX.is_match(label) {
            Ok(Label {
                label: String::from(label)
            })
        } else {
            Err(InputError::new(Some(LABEL_ERROR_MESSAGE)))
        }
    }
    pub fn get(&self) -> String {
        self.label.clone()
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_usernames() {
        vec!["Somebody62", "56_3", "Goodjob2020fffff"]
            .into_iter()
            .for_each(|username| {
                assert!(parse(&USERNAME_REGEX, username).is_some());
            });
    }
    #[test]
    fn bad_usernames() {
        vec!["QQQQQ4%", "e-4", "fdsfjlskdfalsdflajsdlgnoandsg"]
            .into_iter()
            .for_each(|username| {
                assert!(parse(&USERNAME_REGEX, username).is_none());
            });
    }
    #[test]
    fn good_gallery_names() {
        vec!["Somebody62", "56f3", "Goodjob2020____fffff"]
            .into_iter()
            .for_each(|gallery| {
                assert!(parse(&GALLERY_REGEX, gallery).is_some());
            });
    }
    #[test]
    fn bad_gallery_names() {
        vec!["e$e", "fdsfjlskdfalsdflajsdlgnoandsggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg", ""].into_iter().for_each(|gallery| {
            assert!(parse(&GALLERY_REGEX, gallery).is_none());
        });
    }
    #[test]
    fn good_image_names() {
        vec!["Somebody62.jpg", "#DCIM-546_rev.2.jpg", "#.jpg"]
            .into_iter()
            .for_each(|image| {
                assert!(parse(&IMAGETITLE_REGEX, image).is_some());
            });
    }
    #[test]
    fn bad_image_names() {
        vec!["QQQQQ4%.jpg", "e-4.png", "fdsfjlskdfalsdflajsdlgnoandsggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg.jpg"].into_iter().for_each(|image| {
            assert!(parse(&IMAGETITLE_REGEX, image).is_none());
        });
    }
    #[test]
    fn good_passwords() {
        vec![
            "Somebody62",
            "56_3hjklj!@#$%^&*()",
            "Goodjob2020fffffakjdsflkj;",
        ]
        .into_iter()
        .for_each(|password| {
            assert!(parse(&PASSWORD_REGEX, password).is_some());
        });
    }
    #[test]
    fn bad_passwords() {
        vec!["", "e-4", "fdsfjlskdfalsdflajsdlgnoandsgffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"].into_iter().for_each(|password| {
            assert!(parse(&PASSWORD_REGEX, password).is_none());
        });
    }
    #[test]
    fn good_labels() {
        vec![
            "_89@",
            "HIFDFKAFDJSDFKLJFDDjsdfslkfj@__@_@__@_@_@_@_",
            "Goodjob2020fffffakjdsfl",
        ]
        .into_iter()
        .for_each(|label| {
            assert!(parse(&LABEL_REGEX, label).is_some());
        });
    }
    #[test]
    fn bad_labels() {
        vec!["", "FSFJ:", "fdsfjlskdfalsdflajsdlgnoandsgffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"].into_iter().for_each(|label| {
            assert!(parse(&LABEL_REGEX, label).is_none());
        });
    }
    #[test]
    fn good_emails() {
        vec!["f@d.co", "justus@olmmcc.tk", "cool.dude@abcdef.gh.ij"]
            .into_iter()
            .for_each(|email| {
                assert!(parse(&EMAIL_REGEX, email).is_some());
            });
    }
    #[test]
    fn bad_emails() {
        vec![
            "ffdsjk@ccc",
            "fsf@.tk",
            "@col.col",
            "asdfasdfasdfasdfasdfasdfasdfasdfasdf@hi.com",
            "asdf@asdfasdfasdfasdfasdfasfdasdfasdfasdf.com",
            "asdf@asdf.asdfkjf",
        ]
        .into_iter()
        .for_each(|email| {
            assert!(parse(&EMAIL_REGEX, email).is_none());
        });
    }
    #[test]
    fn good_ids() {
        vec!["Bgb3IqYnBrC9MVfvvTW3h2jLd8O7Q0Cz7acpkR17DfPliZJjwpD6yEfgT19M2b6C3pPoOWJSwCmGXlTHmE864D2yWGsAZtegKWK61BwjINRL2br8W1pQC9tYNhZxongAB1TDlzcbIk9NQNJbXneHEx1tQEiiEb651zSQAjvA77QHIVCkOaa6WE2dkwrkVHDaKCCqQ1v1GY73nro6rIUelzQWCrsfdATB2dfuHLbwOXpMq9PEQCpWNaiVVstDuh0"].into_iter().for_each(|id| {
            assert!(parse(&ID_REGEX, id).is_some());
        });
    }
    #[test]
    fn bad_ids() {
        vec!["Bgb3IqYnBrC9MVfvvTW3h2jLd8O7Q0Cz7acpkR17DfPliZJjwpD6yEfgT19M2b6C3pPoOWJSwCmGXlTHmE864D2yWGsAZtegKWK61BwjINRL2br8W1pQC9tYNhZxongAB1TDlzcbIk9NQNJbXneHEx1tQEiiEb651zSQAjvA77QHIVCkOaa6WE2dkwrkVHDaKCCqQ1v1GY73nro6rIUelzQWCrsfdATB2dfuHLbwOXpMq9PEQCpWNaiVVstDuh_", "Bgb3IqYnBrC9MVfvvTW3h2jLd8O7Q0Cz7acpkR17DfPliZJjwpD6yEfgT19M2b6C3pPoOWJSwCmGXlTHmE864D2yWGsAZtegKWK61BwjINRL2br8W1pQC9tYNhZxongAB1TDlzcbIk9NQNJbXneHEx1tQEiiEb651zSQAjvA77QHIVCkOaa6WE2dkwrkVHDaKCCqQ1v1GY73nro6rIUelzQWCrsfdATB2dfuHLbwOXpMq9PEQCpWNaiVVstDuh"].into_iter().for_each(|id| {
            assert!(parse(&ID_REGEX, id).is_none());
        });
    }
}
/*
#[derive(Deserialize)]
pub struct ImageDetails {
    username: String,
    gallery: String,
    image_title: String,
}

impl ImageDetails {
    pub fn get_username(&self) -> Result<String, InputError> {
        Ok(Username::new(&self.username)?.get())
    }
    pub fn get_gallery(&self) -> Result<String, InputError> {
        Ok(Gallery::new(&self.gallery)?.get())
    }
    pub fn get_image_title(&self) -> Result<String, InputError> {
        Ok(ImageTitle::new(&self.image_title)?.get())
    }
}

#[derive(Deserialize)]
pub struct UserDetails {
    username: String,
}

impl UserDetails {
    pub fn get_username(&self) -> Result<String, InputError> {
        Ok(Username::new(&self.username)?.get())
    }
}

#[derive(Deserialize)]
pub struct GalleryDetails {
    username: String,
    gallery: String,
}

impl GalleryDetails {
    pub fn get_username(&self) -> Result<String, InputError> {
        Ok(Username::new(&self.username)?.get())
    }
    pub fn get_gallery(&self) -> Result<String, InputError> {
        Ok(Gallery::new(&self.gallery)?.get())
    }
}
*/

#[derive(Deserialize)]
pub struct Signup {
    email: String,
    password: String,
    username: String,
}

impl Signup {
    pub fn get_email(&self) -> Option<String> {
        parse(&EMAIL_REGEX, &self.email)
    }
    pub fn get_password(&self) -> Option<String> {
        parse(&PASSWORD_REGEX, &self.password)
    }
    pub fn get_username(&self) -> Option<String> {
        parse(&USERNAME_REGEX, &self.username)
    }
}

#[derive(Deserialize)]
pub struct Login {
    email: String,
    password: String,
}

impl Login {
    pub fn get_email(&self) -> Option<String> {
        parse(&EMAIL_REGEX, &self.email)
    }
    pub fn get_password(&self) -> Option<String> {
        parse(&PASSWORD_REGEX, &self.password)
    }
}

#[derive(Deserialize)]
pub struct GalleryCreate {
    gallery_name: String,
    id: String,
}

impl GalleryCreate {
    pub fn get_gallery_name(&self) -> Option<String> {
        parse(&GALLERY_REGEX, &self.gallery_name)
    }
    pub fn get_id(&self) -> Option<String> {
        parse(&ID_REGEX, &self.id)
    }
}
