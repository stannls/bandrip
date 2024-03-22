use std::{env, path::PathBuf};

use bandcamp::TrackMetadata;
use clap::{arg, command};
use dirs::audio_dir;
mod bandcamp;
mod downloader;

fn main() {
    let matches = command!()
        .name("bandip")
        .version("0.1.2")
        .about("A simple bandcamp downloader.")
        .arg(
            arg!(<LINK>)
                .required(true)
                .id("link")
                .help("The bandcamp link to download from."),
        )
        .get_matches();
    let download_links = bandcamp::extract_audio_links(matches.get_one::<String>("link").unwrap());
    if download_links.is_err() {
        println!("Error parsing bandcamp site.");
        return;
    }
    let download_links = download_links.unwrap();
    println!(
        "Starting download. Found {} tracks...",
        download_links.len()
    );
    for (link, metadata) in download_links.to_owned() {
        let downloaded_file = downloader::download_from_link(link).unwrap();
        downloader::move_and_tag_file(downloaded_file, metadata.to_owned()).unwrap();
        println!(
            "Downloaded {} {} by {}",
            metadata.track_number, metadata.name, metadata.artist
        );
    }
    println!(
        "Finished downloading to {:?}",
        get_download_dir(&download_links.get(0).unwrap().1).into_os_string()
    );
}

fn get_download_dir(metadata: &TrackMetadata) -> PathBuf {
    let mut download_dir = audio_dir().unwrap();
    download_dir.push("bandrip");
    download_dir.push(metadata.artist.to_owned());
    download_dir.push(metadata.album.to_owned());
    download_dir
}
