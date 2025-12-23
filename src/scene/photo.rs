use crate::scene::Rect;
use crate::util::datetime::DateTime;
use crate::{error::Result, v2d};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use v2d::m4x4;

#[derive(Clone, Debug)]
pub struct Photo {
    pub path: PathBuf,
    pub meta: PhotoMeta,
}

impl Photo {
    pub fn from_path(path: PathBuf) -> Result<Self> {
        let json_path = path.with_extension("json");
        let data = std::fs::read_to_string(json_path)?;
        let meta = serde_json::from_str(&data)?;
        Ok(Self { path, meta })
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct PhotoMeta {
    pub datetime: Option<DateTime>,
    pub place: Option<Vec<String>>,
    pub title: Option<Vec<String>>,
    pub tag: Option<Vec<String>>,
    pub weather: Option<Vec<String>>,
    pub rating: Option<u8>,
}

fn is_webp_file(path: &Path) -> bool {
    path.is_file()
        && path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("webp"))
}

pub fn read_webp_photos(dir: &Path) -> Vec<Photo> {
    log::info!("Reading photos: {dir:?}");
    let mut photos = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if is_webp_file(&path) {
                let photo = Photo::from_path(path);
                log::info!("Found photo: {:?} => {photo:?}", entry.path());
                if let Ok(photo) = photo {
                    photos.push(photo);
                }
            }
        }
    }
    photos
}

#[rustfmt::skip]
pub fn transform(dst: &Rect) -> m4x4::M4x4 {
    m4x4::M4x4::new([
        dst.size.x0(), 0.0,           0.0, 0.0,
        0.0,           dst.size.x1(), 0.0, 0.0,
        0.0,           0.0,           1.0, 0.0,
        dst.pos.x0(),  dst.pos.x1(),  0.0, 1.0,
    ])
}
