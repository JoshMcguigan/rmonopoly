extern crate rand;

mod player;
use player::Player;

mod cli;
mod dice;

fn main() {
    let number_of_players = cli::get_number_of_players();

    let mut players: Vec<Player> = Vec::new();
    for i in 0..number_of_players {
        let player_name = cli::get_player_name(i);
        players.push(Player::new(player_name));
    }

    'game: loop {
        for mut player in players.iter_mut() {
            cli::printnls(100);
            player.take_turn();

            if !cli::press_enter() { break 'game; }
        }
    }
}
