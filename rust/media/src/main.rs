#![allow(dead_code)]
mod content;
use content::catalog::Catalog;
use content::media::Media;

fn main() {
    let audiobook = Media::Audiobook {
        title: "The Great Gatsby".to_string(),
    };

    let book = Media::Book {
        title: "The Great Gatsby".to_string(),
        author: "F. Scott Fitzgerald".to_string(),
    };

    let movie = Media::Movie {
        title: "The Great Gatsby".to_string(),
        director: "Baz Luhrmann".to_string(),
    };

    let podcast = Media::Podcast { episode: 32 };

    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(book);
    catalog.add(movie);
    catalog.add(podcast);
    catalog.add(placeholder);

    let item = catalog.get_by_index(2);
    match item {
        Some(media) => {
            println!("Found item: {:#?}", media);
        }
        None => {
            println!("Item not found");
        }
    }
}
