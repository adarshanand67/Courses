use rand::{random, seq::SliceRandom, thread_rng, Rng};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    // inherent implementation - adding a method to the Deck struct
    fn new() -> Self {
        // class methods are called associated functions in Rust
        // List of suits - Spades, Hearts, Diamonds, Clubs
        let suits = ["Spades", "Hearts", "Diamonds", "Clubs"];

        // List of values - Ace, 2-10, Jack, Queen, King
        let values = ["A", "2", "J", "Q", "K"];

        // Create a vector to hold the deck of cards
        let mut cards = Vec::new();
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards } // implicit return
    }

    fn shuffle(&mut self) {
        // shuffle the deck of cards
        let mut rng = thread_rng(); // random number generator
        self.cards.shuffle(&mut rng);
    }

    fn Deal(&mut self, num_cards: usize) -> Vec<String> {
        // usize - number of elements
        // deal a number of cards from the deck
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let rng = thread_rng();
    let mut deck = Deck::new();
    deck.shuffle();
    println!("Your deck: {:#?}", deck); // :#? pretty prints the struct

    let hand = deck.Deal(3);
    println!("Your hand: {:#?}", hand);

    println!("Your deck: {:#?}", deck);
}
