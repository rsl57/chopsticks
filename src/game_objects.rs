pub enum Player {
    TwoHands(i32,i32),
    OneHand(i32),
    Out
}

impl Player {
    fn get_values(&self) -> (i32, i32) {
        match self {
            Self::TwoHands(h1,h2) => (*h1, *h2),
            Self::OneHand(h1) => (*h1, 0),
            Self::Out => (0, 0)
        }
    }
}

pub struct GameState {
    player1: Player,
    player2: Player,
    turn: i32
}

impl GameState {
    pub fn as_tuple(&self) -> (i32, i32, i32, i32, i32) {
        let (p1h1, p1h2) = self.player1.get_values();
        let (p2h1, p2h2) = self.player2.get_values();
        (p1h1, p1h2, p2h1, p2h2, self.turn)
    }
}

pub fn new_game() -> GameState {
    GameState {
        player1: Player::TwoHands(1,1),
        player2: Player::TwoHands(1,1),
        turn: 0
    }
}
