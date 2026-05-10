mod decks;
mod errors;
mod hands;

use decks::Deck;
use errors::GameError;
use hands::Hands;

use crate::hands::{print_dealer, print_dealer_draw, print_player, print_player_draw};

pub fn run() -> Result<(), GameError> {
    let mut deck = Deck::new();
    let mut player = Hands::new();
    let mut dealer = Hands::new();

    deck.shuffle();
    player.add(deck.deal()?);
    player.add(deck.deal()?);
    dealer.add(deck.deal()?);
    dealer.add(deck.deal()?);
    print_player(&player);
    print_dealer(&dealer);

    loop {
        println!("Hit or stand? (h/s)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        match input.trim() {
            "h" | "hit" => {
                let card = deck.deal()?;
                print_player_draw(&card);
                player.add(card);
                print_player(&player);
                if player.score() > 21 {
                    println!("Bust! You lose.");
                    return Ok(());
                }
            }
            "s" | "stand" => break,
            _ => println!("Error input. Please retry."),
        }
    }

    print_dealer(&dealer);
    while dealer.score() < 17 {
        let card = deck.deal()?;
        print_dealer_draw(&card);
        dealer.add(card);
        print_dealer(&dealer);
    }

    if dealer.score() > 21 {
        println!("Dealer busts! You win.");
        return Ok(());
    }

    let player_score = player.score();
    let dealer_score = dealer.score();
    println!("Final - You: {}, Dealer: {}", player_score, dealer_score);
    if player_score > dealer_score {
        println!("You win!");
    } else if dealer_score > player_score {
        println!("Dealer wins!");
    } else {
        println!("Push! It's a tie.");
    }

    Ok(())
}
