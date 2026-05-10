use crate::decks::Card;

pub struct Hands<T> {
    cards: Vec<T>,
}

pub fn print_player<T: Card>(player: &Hands<T>) {
    println!(
        "Your hand: {} (score: {})",
        player.display(),
        player.score()
    );
}

pub fn print_dealer<T: Card>(dealer: &Hands<T>) {
    println!(
        "Dealer shows: {} (score: {})",
        dealer.display(),
        dealer.score()
    );
}

pub fn print_player_draw<T: Card>(card: &T) {
    println!("You drew: {}", card.display());
}

pub fn print_dealer_draw<T: Card>(card: &T) {
    println!("Dealer drew: {}", card.display());
}


impl<T: Card> Hands<T> {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn add(&mut self, card: T) {
        self.cards.push(card);
    }

    pub fn score(&self) -> u8 {
        let mut total = 0;
        let mut aces = 0;
        for card in &self.cards {
            let value = card.value();
            total += value;
            if value == 11 {
                aces += 1;
            }
        }

        while total > 21 && aces > 0 {
            total -= 10;
            aces -= 1;
        }
        total
    }

    pub fn display(&self) -> String {
        self.cards
            .iter()
            .map(|c| c.display())
            .collect::<Vec<_>>()
            .join(", ")
    }
}
