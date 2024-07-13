/*
make a game like black jack
- with 6 steps
    - prepare a deck as a list of cards like vector<vector<number, text, score>>
        - number (like 1-13)
        - card text(like ♠1)
        - score (1-10)
    - shuffle the deck
    - computer and user draw a card from the deck and calculate the score
    - user choose to draw a card or not
    - computer draw a card until the score is over 17
    - compare the score and decide the winner
*/

// extern crate rand;
use rand::prelude::SliceRandom;
use std::io;

enum Card {
    Number(i32),
    Text(String),
    Score(i32)
}

//make this with for loop
//- prepare a deck as a list of cards like list<vector<number, text, score>>
//
fn prepare_deck() -> Vec<Card> {
    let mut deck = Vec::new();
    let number = vec![1,2,3,4,5,6,7,8,9,10,11,12,13];
    let text_mark = vec!["♠", "♣", "♥", "♦"];
    let text_number = vec!["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];
    for i in 0..text_mark.len() {
        for j in 0..number.len() {
            deck.push(Card::Number(number[j]));
            // add text like "♠1"
            deck.push(Card::Text(text_number[i].to_owned() + &text_mark[i].to_owned()));
            // add score as number but over 10 is 10
            if number[j] <= 10 {
                deck.push(Card::Score(10));
            } else {
                deck.push(Card::Score(number[j]));
            }
        }
    }
    deck
}

// - shuffle the deck
fn shuffle_deck(mut deck: Vec<Card>) -> Vec<Card> {
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);
    deck
}


// take given num cards from the deck and add to the hand
fn draw_card(deck: &mut Vec<Card>, num: i32) -> Vec<Card> {
    let mut hand = Vec::new();
    for _ in 0..num {
        hand.push(deck.pop().unwrap());
    }
    hand
}

// print sll decks for debug
fn print_deck(deck: Vec<Card>) {
    for i in 0..deck.len() {
        match &deck[i] {
            Card::Text(t) => println!("Text:   {}", t),
            Card::Number(n) => println!("Number: {}", n),
            Card::Score(s) => println!("Score:  {}", s),
        }
    }
}

// print hand by given number or if number is not given then print all
fn print_hand(hand: Vec<Card>, num: usize) {
    for i in 0..num {
        match &hand[i] {
            Card::Text(t) => println!("Text:   {}", t),
            Card::Number(n) => println!("Number: {}", n),
            Card::Score(s) => println!("Score:  {}", s),
        }
    }
}

//print the score of the hand in given number or if number is not given then print all
fn print_score(hand: Vec<Card>, num: usize) {
    let mut score = 0;
    for i in 0..num {
        match &hand[i] {
            Card::Score(s) => score += s,
            _ => (),
        }
    }
    println!("Score: {}", score);
}

fn main() {
    println!("let's play black jack!");

    let deck = prepare_deck();
    let deck = shuffle_deck(deck);
    // print_deck(deck);

    // computer draw two cards
    let computer_hand = draw_card(deck, 2);
    // user draw two cards
    let user_hand = draw_card(deck, 2);
    //show dealers 1st card and users 2 cards
    println!("Dealer's hand:");
    print_hand(computer_hand, 1);
    // show score of the dealer
    println!("Dealer's score:");
    print_score(computer_hand, 1);
    //show the score of the user
    println!("Your hand:");
    print_hand(user_hand, 2);
    println!("Your score:");
    print_score(user_hand, 2);

    // ask user to draw a card or not
    let mut choice = String::new();
    println!("Do you want to draw a card? (y/n)");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    println!("You chose: {}", choice);

    // if user chose to draw a card draw a card
    if choice.trim() == "y" {
        let user_hand = draw_card(deck, 1);
        println!("Your hand:");
        print_hand(user_hand, 3);
        println!("Your score:");
        print_score(user_hand, 3);
    }

    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");     // 行の読み込みに失敗しました

    // println!("You guessed: {}", guess);     // 次のように予想しました: {}
}
