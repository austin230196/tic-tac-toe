

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

    pub fn board_status(&self) -> &[[Option<Card>; 3]] {
        &self.board
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

    pub fn play(&mut self, entry: Vec<u32>, card: Option<Card>){
        self.board.board[(entry[0] as usize) - 1][(entry[1] as usize) - 1] = card;
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

    pub fn add_player(mut self, player: Player) -> Result<Game, &'static str> {
        if self.players.len() == 2 {
            return Err("Can't add another player to an already full game");
        }
        self.players.push(player);
        Ok(self)
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