use std::env;
mod bandcamp;
mod downloader;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please enter argument containing bandcamp page link.");
    } else {
        let download_links = bandcamp::extract_audio_links(args.get(1).unwrap()).expect("Error parsing bandcamp site.");
        println!("Starting download. Found {} tracks...", download_links.len());
        for (link, metadata) in download_links {
            let downloaded_file = downloader::download_from_link(link).unwrap();
            downloader::move_and_tag_file(downloaded_file, metadata.to_owned()).unwrap();
            println!("Downloaded {} {} by {}", metadata.track_number, metadata.name, metadata.artist);
        }
    }
}
