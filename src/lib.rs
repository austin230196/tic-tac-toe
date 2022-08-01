

#[derive(Debug)]
pub struct GameBoard {
    board: [[Option<Card>; 3]; 3]
}

impl GameBoard {
    pub fn new() -> Self {
        Self {
            board: [[None; 3]; 3]
        }
    }

    pub fn board_status(&self) -> &[[Option<Card>; 3]; 3] {
        &self.board
    }

    pub fn get_available_spaces(&self) -> Vec<(u8, u8)> {
        let mut v = vec![];
        for (i, r) in self.board.iter().enumerate() {
            for (j, c) in r.iter().enumerate() {
                if c.is_none() {
                    let t = ((i+ 1) as u8, (j+ 1) as u8);
                    v.push(t);
                }else {continue;}
            }
        }
        v
    }

    pub fn get_user_board_entries(&self, card: &Option<Card>) -> Vec<(u8, u8)> {
        let mut v = vec![];
        for (i, r) in self.board.iter().enumerate() {
            for (j, c) in r.iter().enumerate() {
                if c == card {
                    let t = ((i + 1) as u8, (j + 1) as u8);
                    v.push(t);
                }else {continue;}
            }
        }
        v
    }

    pub fn get_winning_patterns(&self) -> Vec<((u8, u8), (u8, u8), (u8, u8))>{
        let mut v = vec![];
        v.push(((1, 1), (1, 2), (1, 3)));
        v.push(((1, 1), (2, 1), (3, 1)));
        v.push(((1, 1), (2, 2), (3, 3)));
        v.push(((2, 1), (2, 2), (2, 3)));
        v.push(((3, 1), (3, 2), (3, 3)));
        v.push(((1, 3), (2, 2), (3, 1)));
        v.push(((1, 2), (2, 2), (3, 2)));
        v.push(((1, 3), (2, 3), (3, 3)));
        v
    }
}




#[derive(Debug)]
pub struct Game {
    pub board: GameBoard,
    pub players: Vec<Player>,
    pub bot: bool,
    pub turn: usize,
    pub status: GameStatus
}

impl Game {
    pub fn new(board: GameBoard, add_bot: bool) -> Self {
        //check if add_bot is true
        let mut players = vec![];
        if add_bot {
            //create a bot player
            let bot = Player::new();
            players.push(bot);
        }
        Self {
            board,
            players,
            bot: add_bot,
            turn: 0,
            status: GameStatus::Creating
        }
    }

    pub fn play(&mut self, entry: (u8, u8)){
        let (r, c) = entry;
        self.board.board[(r - 1) as usize][(c - 1) as usize] = self.players[self.turn].card;
    

        //check if the user has won
        let patterns = self.board.get_winning_patterns();
        let user_entries = self.board.get_user_board_entries(&self.players[self.turn].card);

        let mut change_turn = || {
            if self.turn == 0 {
                self.turn = 1;
            }else {
                self.turn = 0;
            }
        };


        //user entry must be 3 to be able to qualify as a winner
        if user_entries.len() < 3 {
            //move the turn to the next user
            change_turn()
        }else {
            //at this point the entry by the user is 3 or more so we check if the user has won
            //filter out the patterns that dont match
            let p = patterns.clone();
            for (j, i) in patterns.into_iter().enumerate() {
                let (f, s, t) = i;
                let i = vec![f, s, t];
                let is_last = || -> bool {
                    p.len() - 1 == j
                };
                match user_entries.as_slice() {
                    [first, second, third] => {
                        //at this point only 3 entries were received
                        if !i.contains(first) && !i.contains(second) && !i.contains(third) {
                            //change turn
                            if !is_last() {
                                continue;
                            }else {
                                change_turn();
                                break;
                            }
                        }else if i.contains(first) && i.contains(second) && i.contains(third) {
                            //at this point the user has won
                            println!("Player {:?} has won", self.players[self.turn]);
                            self.status = GameStatus::Finished;
                            break;
                        }else {
                            if !is_last() {
                                continue;
                            }else {
                                change_turn();
                                break;
                            }
                        }
                    },
                    [first, second, third, fourth] => {
                        //at this point the user entered more than 3 entries
                        if (i.contains(first) && i.contains(second) && (i.contains(third) || i.contains(fourth))) || 
                        (i.contains(third) && i.contains(fourth) && (i.contains(first) || i.contains(second))) {
                            //at this point the user has won
                            println!("Player {:?} has won", self.players[self.turn]);
                            self.status = GameStatus::Finished;
                            break;
                        }else {
                            if !is_last() {
                                continue;
                            }else {
                                change_turn();
                                break;
                            }
                        }
                    },
                    entry @ _ => {
                        println!("Entries: {:?}", entry);
                    }
                }
            }
        }
    }

    pub fn start(&mut self) -> bool {
        if self.status == GameStatus::Playing {
            false
        }else if self.status == GameStatus::Finished {
            false
        }else {
            //at this point the status of the game is creating 
            //so we switch it
            //check if the players are already complete
            if self.players.len() != 2 {
                return false;
            }
            self.status = GameStatus::Playing;
            true
        }
    }

    pub fn add_player(&mut self, player: Player) -> bool {
        if self.players.len() == 2 {
            return false;
        }
        self.players.push(player);
        true
    }


    pub fn get_position_of_user_with_no_card(&self) -> Option<usize> {
        self.players.iter().position(|p| p.card == None)
    }


    pub fn available_cards(&self) -> Option<Vec<Card>> {
        if self.players.len() == 0 {
            //no players exist so all cards are available
            Some(Vec::from([Card::X, Card::O]))
        }else if self.players.len() == 1 {
            //get the users card 
            let user_card = self.players[0].card;
            if let None = user_card {
                Some(Vec::from([Card::X, Card::O]))
            }else {
                //at this point the uer found has a card
                if user_card.unwrap() == Card::X {
                    Some(Vec::from([Card::O]))
                }else {
                    Some(Vec::from([Card::X]))
                }
            }

        }else {
            //at this point the players length is 2
            if self.players[0].card.is_none() && self.players[1].card.is_none() {
                Some(Vec::from([Card::X, Card::O]))
            }else if self.players[0].card.is_none() && self.players[1].card.is_some() {
                if self.players[1].card.unwrap() == Card::X {
                    Some(Vec::from([Card::O]))
                }else {
                    Some(Vec::from([Card::X]))
                }
            } else if self.players[0].card.is_some() && self.players[1].card.is_none() {
                if self.players[0].card.unwrap() == Card::X {
                    Some(Vec::from([Card::O]))
                }else {
                    Some(Vec::from([Card::X]))
                }
            } else {
                //at this point the cards has been taken that is the condition
                // self.players[0].card.is_some() && self.players[1].card.is_some();
                None
            }
            
        }
    }
}


#[derive(Debug)]
pub struct Player {
    pub card: Option<Card>,
}


impl Player {
    pub fn new() -> Self {
        Self {
            card: None
        }
    }

    pub fn set_card(&mut self, card: Option<Card>){
        self.card = card;
    }
}




#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Card {
    X,
    O
}



#[derive(PartialEq, Debug)]
pub enum GameStatus {
    Playing,
    Creating,
    Finished
}