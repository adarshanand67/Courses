
#[derive(Debug, Clone)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast { episode: u32 },
    Placeholder,
}

impl Media {
    pub fn description(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book: {} by {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} by {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
            Media::Podcast { episode } => {
                format!("Podcast: Episode {}", episode)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

pub fn print_media(media: Media) {
    match media {
        Media::Book { title, author } => {
            println!("Book: {} by {}", title, author);
        }
        Media::Movie { title, director } => {
            println!("Movie: {} by {}", title, director);
        }
        Media::Audiobook { title } => {
            println!("Audiobook: {}", title);
        }
        Media::Podcast { episode } => {
            println!("Podcast: Episode {}", episode);
        }
        Media::Placeholder => {
            println!("Placeholder");
        }
    }
}