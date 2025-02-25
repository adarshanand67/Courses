use super::media::{print_media, Media};

#[derive(Debug)]
pub struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    pub fn new() -> Catalog {
        Catalog { items: Vec::new() }
    }

    pub fn add(&mut self, item: Media) {
        self.items.push(item);
    }

    pub fn display(&self) {
        for item in &self.items {
            print_media(item.clone());
        }
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Media> {
        if index < self.items.len() {
            Some(&self.items[index])
        } else {
            None
        }
    }
}