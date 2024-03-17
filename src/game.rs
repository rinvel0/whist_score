use std::io;
#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub score: i16,
    pub bid: i16,
    pub hands: i16,
    pub prize: i16,
}

impl Player {
    fn update_score(&mut self) {
        if self.bid == self.hands {
            self.score += 5;
            self.score += self.bid;
        } else {
            self.score -= (self.bid - self.hands).abs();
        }
    }
    fn update_hands(&mut self, hands: i16) {
        self.hands = hands;
    }
    fn update_bid(&mut self, bid: i16) {
        self.bid = bid;
    }
}

pub struct GameState {
    pub players: Vec<Player>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            players: Vec::new(),
        }
    }
    pub fn add_player(&mut self, name: String) {
        let player = Player {
            name: name,
            score: 0,
            bid: 0,
            hands: 0,
            prize: 0,
        };
        self.players.push(player);
    }
    pub fn update_score(&mut self, player_index: i16) {
        // Update the score
        // to copy from the old method of player.
    }
}

// Function that reads the number of players
pub fn num_of_players() -> i16 {
    loop {
        println!("Please enter the number of players: ");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<i16>() {
            Ok(n) if (3..=6).contains(&n) => {
                return n;
            }
            Ok(_) => {
                println!("The number of players must bee between 3 and 6! Try again.");
            }
            Err(_) => {
                println!("That's not a number. Please enter a valid number.");
            }
        }
    }
}

// Function that creates a vector of players
pub fn create_players(num_players: i16) -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();

    for i in 1..=num_players {
        println!("Enter the name of player {}", i);

        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");

        let player = Player {
            name: name.trim().to_string(),
            score: 0,
            bid: 0,
            hands: 0,
            prize: 0,
        };
        players.push(player);
    }

    players
}

//Function for playing round
pub fn play_round(players: &mut Vec<Player>, num_cards: i16, starting_player_index: usize) {
    // Deal num_cards

    // Dealer is ?
    let dealer_index = if starting_player_index == 0 {
        players.len() - 1
    } else {
        starting_player_index - 1
    };
    let dealer = &players[dealer_index];
    println!("The dealer for this round: {}", dealer.name);
    // First player is ?
    let starting_player = &players[starting_player_index];
    println!(
        "The starting player for this round is: {}",
        starting_player.name
    );

    // Each player bids

    let mut total_no_of_hands = 0;
    for i in 0..=players.len() - 1 {
        let player_index = (starting_player_index + i) % players.len();
        if i == players.len() - 1 && total_no_of_hands <= num_cards {
            let not_allowed = (total_no_of_hands - num_cards).abs();
            println!(
                "{} bids, not allowed {}: ",
                &players[player_index].name, not_allowed
            );
            let bid = read_validate_bid(num_cards, Some(not_allowed));
            players[player_index].update_bid(bid);
            total_no_of_hands += &bid;
        } else {
            println!("{} bids: ", &players[player_index].name);
            let bid = read_validate_bid(num_cards, None);
            players[player_index].update_bid(bid);
            total_no_of_hands += &bid;
        }
    }
    println!("Total number of bids: {}", total_no_of_hands);

    // println!("{:#?}", players);
    // Play the game
    //...

    // Record hands
    for i in 0..=players.len() - 1 {
        let player_index = (starting_player_index + i) % players.len();
        loop {
            let mut hands = String::new();
            println!("{} took: ", &players[player_index].name);
            io::stdin()
                .read_line(&mut hands)
                .expect("Failed to read line");
            match hands.trim().parse() {
                Ok(n) if n <= num_cards => {
                    players[player_index].update_hands(n);
                    break;
                }
                Ok(_) => {
                    println!("Bid out of range. Must be between 0 and {}", &num_cards);
                }
                Err(e) => {
                    println!("That is not a number. try again. {e}");
                }
            }
        }
    }
    // Scoring
    for i in 0..=players.len() - 1 {
        let player_index = (starting_player_index + i) % players.len();
        players[player_index].update_score();
    }

    // Cleanup
}

// Read and Validate bid
pub fn read_validate_bid(num_cards: i16, not_allowed: Option<i16>) -> i16 {
    loop {
        let mut bid = String::new();
        io::stdin()
            .read_line(&mut bid)
            .expect("Failed to read line");
        match bid.trim().parse() {
            Ok(n) if n <= num_cards && n != not_allowed.unwrap_or(100) => {
                return n;
            }
            Ok(_) => {
                println!(
                    "Bid out of range or not allowed. Must be between 0 and {}",
                    &num_cards
                );
            }
            Err(e) => {
                println!("That is not a number. try again. {e}");
            }
        }
    }
}
