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
    pub fn print_board(game: &Game){
        let board = game.board.board_status();
        for b in board {
            //array destructuring
            let [c, d, e] = b;
            println!("{:?} | {:?} | {:?} \n", c, d, e);
        }
    }

    
    ///function for getting putting the available_entries in a hashmap
    pub fn get_available_entries(game: &Game){
        println!("Please pick an entry to enter card");
        let board = game.board.board_status();
        let mut map = HashMap::new();
        for (i, b) in board.iter().enumerate() {
            for (j, c) in b.iter().enumerate() {
                if c.is_none() {
                    let tup = ((i as u32) + 1, (j as u32) + 1);
                    map.insert(tup, c);
                }else {continue;}
            }
        }
        for (key, card) in map.into_iter() {
            let (k, v) = key;
            println!("{k},{v}: {:?}", card);
        }
    }


    ///this function prepares the game plus adds the players
    pub fn setup_game(bot: bool, chosen: Option<Card>) -> Game {
        //we create a new Gameboard
        let gameboard = GameBoard::new();

        //then we create a Game
        let mut newgame = Game::new(gameboard, bot);

        //create the player
        let mut player = Player::new();

        //add the player to the game
        player.set_card(chosen);
        let has_added_player = newgame.add_player(player);
        if !has_added_player {
            panic!("#main: Unexpected error adding player to the card");
        }


        //get the available card
        let available_card = newgame.available_cards().expect("#main: Unexpected error getting the available cards")[0];
        
        //then we get the position of the user with no card
        let position = newgame.get_position_of_user_with_no_card().expect("#main: Unexpected error getting the position of the user with empty card");

        //then assign the remaining card to the user
        newgame.players[position].set_card(Some(available_card));

        newgame

    }


    //check the pattern of the user's input
    pub fn check_user_entry_pattern(game: &Game, input: String) -> Option<(u8, u8)>{
        let input = input.trim();
        if input.is_empty() {
            println!("Input cannnot be empty please enter a valid input");
            None
        }else if !input.contains(","){
            println!("Wrong input format (row number, column number)");
            None
        }else {
            let mut e = input.split(",");
            let row = e.next();
            let column = e.next();
            if row.unwrap().trim().is_empty() || row.is_none() {
                println!("Invalid row entered \n {} is invalid", row.unwrap());
                return None;
            }
            if column.unwrap().trim().is_empty() || column.is_none() {
                println!("Invalid column entered \n {} is invalid", column.unwrap());
                return None;
            }

            //at this point the row and column entered is okay
            let row = row.unwrap().parse::<u8>();
            let column = column.unwrap().parse::<u8>();

            //now check if the entry was an integer
            if row.is_err() {
                println!("Row must be a valid integer");
                return None;
            }
            if column.is_err() {
                println!("Column must be a valid integer");
                return None;
            }

            //at this poin the entry is a valid integer
            let row = row.unwrap();
            let column = column.unwrap();
            println!("Row: {row}, Column: {column}");
            let entry = (row, column);
            let available_entries = game.board.get_available_spaces();
            if available_entries.contains(&entry) {
                return Some(entry);
            }else {
                println!("{:?} is not available", entry);
                return None;
            }
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

        if chosen.is_some() {
            break;
        }
    }

    //at this point the chosen card is valid
    //setup game
    let mut newgame = helpers::setup_game(true, chosen);

    //then we start the game
    let _ = newgame.start();

    //this is the loop of a game that has started
    while newgame.status != GameStatus::Finished {
        //we need to print out the board to the user
        helpers::print_board(&newgame);

        //get the players turn and print it to the console
        let user_turn = &newgame.players[newgame.turn];
        println!("It's player {:?} turn", user_turn);
    
        //print available entries to the user
        helpers::get_available_entries(&newgame);

        //then now we read the users input
        let user_input = helpers::get_user_input();

        //authenticate user entry
        let slot_chosen = helpers::check_user_entry_pattern(&newgame, user_input);
        if slot_chosen.is_none() {
            continue;
        } else {
            //at this point the user entered the right entry
            //so we play it on the board
            newgame.play(slot_chosen.unwrap());
            helpers::print_board(&newgame);
        }
    }
}