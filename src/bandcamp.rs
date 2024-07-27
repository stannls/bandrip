use scraper::{Html, Selector};
use std::io::Error;

pub fn extract_audio_links(
    page_link: &str,
) -> Result<Vec<(String, TrackMetadata)>, Box<dyn std::error::Error + Send + Sync>> {
    let html_page = reqwest::blocking::get(page_link)?.text()?;
    let dom = Html::parse_document(&html_page);
    let selector = Selector::parse("script[data-tralbum]").unwrap();
    let element = dom
        .select(&selector)
        .next()
        .ok_or(Error::new(std::io::ErrorKind::Other, "Parsing failed"))?;
    let mut links = vec![];
    for (name, value) in &element.value().attrs {
        if &name.local == "data-tralbum" {
            let data = gjson::get(value.trim(), "@this");
            let artist = data.get("artist").to_string();
            let album = data.get("current").get("title").to_string();
            links = data
                .get("trackinfo")
                .array()
                .iter()
                .map(|f| {
                    (
                        f.get("file.mp3-128").to_string(),
                        TrackMetadata {
                            name: f.get("title").to_string(),
                            track_number: u16::from_str_radix(&f.get("track_num").to_string(), 10)
                                .unwrap(),
                            artist: artist.to_owned(),
                            album: album.to_owned(),
                        },
                    )
                })
                .collect::<Vec<(String, TrackMetadata)>>();
        }
    }
    Ok(links)
}

#[derive(Debug, Clone)]
pub struct TrackMetadata {
    pub name: String,
    pub track_number: u16,
    pub artist: String,
    pub album: String,
}
