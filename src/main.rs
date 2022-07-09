use std::{io, collections::HashMap};

mod lib;


use lib::*;



mod helpers {
    use super::*;

    ///function prepares the cards in a data stucture
    pub fn prepare_card() -> HashMap<u32, Card> {
        let mut card_selection: HashMap<u32, Card> = HashMap::new();
        card_selection.entry(1).or_insert(Card::X);
        card_selection.entry(2).or_insert(Card::O);
        card_selection
    } 
    
    ///function for printing the card interactively to the console
    pub fn print_cards_to_console(cards: &HashMap<u32, Card>){
        println!("Pick a card \n");
        for i in cards {
            let (key, value) = i;
            println!("{key}: {:#?} \n", value);
        }
    }


    ///function for processing the input of the user and returning it
    pub fn get_user_input() -> String {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("#get_user_input: Failed to read user input");
        user_input
    }
    


    ///function checks if the user entered a valid card
    pub fn check_user_card<'a>(cards: HashMap<u32, Card>, user_input: String) ->  Option<Card> {
        //parse it to u32
        let parsed = user_input.trim().parse::<u32>().expect("#check_user_card: Invalid character entered. \n Please enter a number");
        let mut card = None;
        for (key, value) in cards.into_iter() {
            if key == parsed {
                card = Some(value);
            }else {continue;}
        }
        card
    }


    ///function prints out the board
    pub fn print_board(board: &[[Option<Card>; 3]]){
        for b in board {
            //array destructuring
            let [c, d, e] = b;
            println!("{:#?} | {:#?} | {:#?} \n", c, d, e);
        }
    }

    
    ///function for getting putting the available_entries in a hashmap
    pub fn get_available_entries(board: &[[Option<Card>; 3]]) -> HashMap<(u32, u32), &Option<Card>> {
        let mut map = HashMap::new();
        for (i, b) in board.iter().enumerate() {
            for (j, c) in b.iter().enumerate() {
                if c.is_none() {
                    let tup = ((i as u32) + 1, (j as u32) + 1);
                    map.insert(tup, c);
                }else {continue;}
            }
        }
        map
    }


    pub fn print_entries(entries: HashMap<(u32, u32), &Option<Card>>){
        for (key, value) in entries.into_iter() {
            let (row, column) = key;
            println!("{row},{column}: {:#?}", value);
        }
    }
    
}


fn main() {
    println!("Welcome, Player to X and O");
    //create the cards in a data structure
    let mut cards = helpers::prepare_card();

    //then print it to the console so a player can choose
    helpers::print_cards_to_console(&cards);

    //then we get the user input and panic if error reading input
    let mut user_input = helpers::get_user_input();

    //then we check if the user picked a valid card
    let mut chosen = helpers::check_user_card(cards, user_input);
    println!("{:#?}", &chosen);

    while chosen.is_none() {
        println!("Please pick an available card: X or O");
        //create the cards in a data structure
        cards = helpers::prepare_card();
    
        //then print it to the console so a player can choose
        helpers::print_cards_to_console(&cards);
    
        //then we get the user input and panic if error reading input
        user_input = helpers::get_user_input();
    
        //then we check if the user picked a valid card
        chosen = helpers::check_user_card(cards, user_input);
        println!("{:#?}", &chosen);

        if chosen.is_some() {
            break;
        }
    }

    //at this point the chosen card is valid
    //we create a new Gameboard
    let gameboard = GameBoard::new();

    //then we create a Game
    let newgame = Game::new(gameboard, true);

    //create the player
    let mut player = Player::new();
    player.set_card(chosen);
    let mut newgame = newgame.add_player(player).expect("#main: Unexpected error adding player to the card");

    //get the available card and assign it to the bot
    let available_card = newgame.available_cards().expect("#main: Unexpected error getting the available cards")[0];
    let position = newgame.get_position_of_user_with_no_card().expect("#main: Unexpected error getting the position of the user with empty card");
    newgame.players[position].set_card(Some(available_card));

    todo!();
    ///REFACTOR LOGIC INTO TINY FUNCTIONS

    //then we start the game
    let started = newgame.start();

    //get the available boards
    let board = newgame.board.board_status();

    //at this point the game has started
    //get available entries
    let entries = helpers::get_available_entries(board);

    //we need to print out the board to the user
    helpers::print_board(board);

    //print out available entries
    helpers::print_entries(entries);
    let mut entry = String::new();
    let mut entry_arr;
    println!("Pick an entry to play your card \n");
    io::stdin().read_line(&mut entry).expect("Failed to read user input");
    entry_arr = entry.trim().split(",").map(|e| String::from(e).parse().unwrap_or_else(|_| 0)).collect::<Vec<u32>>();
    while entry_arr.len() != 2 || entry_arr[0] == 0 {
        entry = String::new();
        println!("Pick an entry to play your card \n");
        io::stdin().read_line(&mut entry).expect("Failed to read user input");
        entry_arr = entry.trim().split(",").map(|e| String::from(e).parse().unwrap_or_else(|_| 0)).collect::<Vec<u32>>();
    }
    println!("{:#?}", entry_arr);
    newgame.play(entry_arr, newgame.players[newgame.turn].card);
    println!("the game is {:?} with available cards : {:#?}, empty card position: {:#?} has started ? {:#?}", newgame, available_card, position, started);
}