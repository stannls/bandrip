use audiotags::{AudioTagEdit, AudioTagWrite, Id3v2Tag};
use dirs::audio_dir;
use rand::prelude::*;
use std::io::Cursor;
use std::{
    fs::{self, File},
    path::Path,
};

use crate::bandcamp::TrackMetadata;

pub fn download_from_link(
    link: String,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let response = reqwest::blocking::get(link)?.bytes()?;

    // Rng used for creating a temp filename
    let filename = rand::thread_rng().gen_range(10000..99999);
    let path = format!("/tmp/{}.mp3", filename);
    let mut file = File::create(&path)?;
    let mut content = Cursor::new(response);
    std::io::copy(&mut content, &mut file)?;
    Ok(path)
}

pub fn move_and_tag_file(
    filename: String,
    metadata: TrackMetadata,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let old_file = Path::new(&filename);
    let mut file_dir = audio_dir().unwrap();
    let mut tags = Id3v2Tag::new();
    tags.set_title(&metadata.name);
    tags.set_artist(&metadata.artist);
    tags.set_album_title(&metadata.album);
    tags.set_track_number(metadata.track_number);
    tags.write_to_path(&filename)?;

    file_dir.push("bandrip");
    file_dir.push(metadata.artist.to_owned());
    file_dir.push(metadata.album.to_owned());
    fs::create_dir_all(file_dir.to_str().unwrap())?;
    let filename: String = format!(
        "{} - {} - {} {}.mp3",
        metadata.artist, metadata.album, metadata.track_number, metadata.name
    )
    .chars()
    // Escpae slashes
    .map(|c| if c == '/' { '|' } else { c })
    .collect();
    file_dir.push(filename);

    // Using copy and delete here instead of rename because rename will fail when the
    // destination is on a different mount
    fs::copy(old_file, file_dir)?;
    fs::remove_file(old_file)?;
    Ok(())
}
