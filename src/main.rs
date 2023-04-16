use rand::Rng;

struct Player {
    name: String,
    is_computer: bool,
}

impl Player {
    fn new(name: String, is_computer: bool) -> Self {
        Self {
            name,
            is_computer,
        }
    }
}

struct NimGame {
    current_player: Player,
    total_sticks: u32,
}

impl NimGame {
    fn new(player1: Player, player2: Player, total_sticks: u32) -> Self {
        Self {
            current_player: player1,
            total_sticks,
        }
    }

    fn is_game_over(&self) -> bool {
        self.total_sticks == 0
    }

    fn take_sticks(&mut self, num_sticks: u32) -> Result<(), String> {
        if num_sticks == 0 || num_sticks > 3 || num_sticks > self.total_sticks {
            return Err(String::from("Invalid number of sticks!"));
        }

        self.total_sticks -= num_sticks;
        self.switch_players();
        Ok(())
    }

    fn switch_players(&mut self) {
        if self.current_player.is_computer {
            self.current_player = Player::new(String::from("Human"), false);
        } else {
            self.current_player = Player::new(String::from("Computer"), true);
        }
    }
}

fn main() {
    let player1 = Player::new(String::from("Human"), false);
    let player2 = Player::new(String::from("Computer"), true);
    let total_sticks = rand::thread_rng().gen_range(4..=20);
    let mut game = NimGame::new(player1, player2, total_sticks);

    while !game.is_game_over() {
        println!("Total sticks: {}", game.total_sticks);
        println!("Current player: {}", game.current_player.name);

        let num_sticks = if game.current_player.is_computer {
            // Computer logic for taking sticks goes here
            // For example, the computer could take a random number of sticks between 1 and 3
            let random = rand::thread_rng().gen_range(1..=3);
            random
        } else {
            println!("How many sticks do you want to take?");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");

            input.trim().parse().unwrap()
        };
        println!("Computer takes: {} sticks", num_sticks);
        match game.take_sticks(num_sticks) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }

    println!("Game over!");
    println!("Winner: {}", game.current_player.name);

}