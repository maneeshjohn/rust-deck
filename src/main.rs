use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Deck {
        let suits = ["Hearts", "Clubs", "Spades", "Diamonds"];
        let values = [
            "Ace",
            "Two",
            "Three",
            "Four",
            "Five",
            "Six",
            "Seven",
            "Eight",
            "Nine",
            "Ten",
            "Jack",
            "Queen",
            "King"
        ];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        return Deck{ cards };
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        return self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, size: usize) -> Vec<String> {
        return self.cards.split_off(self.cards.len() - size);
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let hand = deck.deal(4);
    print!("Hand: {:?}", hand);
    print!("Deck: {:?}", deck);
}
