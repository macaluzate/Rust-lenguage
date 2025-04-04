use rand::{thread_rng, seq::SliceRandom}; // importanción del Crate

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];
        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) { // Metodo que mezcla utilizando rand
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize ) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)


    }
}

fn main() {
    let mut deck = Deck::new();
    //deck.shuffle();
    println!("Here is my deck {:#?}", deck);
    let cards = deck.deal(3);

    
    println!("Here is my hand {:#?}", cards);
    println!("Here is my deck {:#?}", deck);
}
